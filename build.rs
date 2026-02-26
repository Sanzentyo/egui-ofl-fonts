use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "src/font_manifest.rs"]
mod font_manifest;

use font_manifest::{FontFamilySpec, LicenseKind};

#[derive(Clone, Debug)]
struct AssetSpec {
    url: String,
    out_name: String,
    fallback_rel_path: String,
    repo_rel_path: String,
}

#[derive(Clone, Debug)]
struct ResolvedFace {
    egui_name: String,
    file_name: String,
}

#[derive(Clone, Debug)]
struct BuildFamilySpec {
    display_name: String,
    google_ofl_dir: String,
    local_family_dir: String,
    specimen_url: String,
    repo_url: String,
}

#[derive(Clone, Debug)]
struct ResolvedFamily {
    display_name: String,
    specimen_url: String,
    repo_url: String,
    faces: Vec<ResolvedFace>,
}

#[derive(Default)]
struct AcquisitionStats {
    total: usize,
    from_cache: usize,
    from_submodule: usize,
    from_local_fallback: usize,
    from_api: usize,
}

enum AssetSource {
    Cache,
    Submodule,
    LocalFallback,
    Api,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/font_manifest.rs");
    println!("cargo:rerun-if-changed=fonts");
    println!("cargo:rerun-if-changed=licenses");
    println!("cargo:rerun-if-changed=.gitmodules");
    println!("cargo:rerun-if-env-changed=GOOGLE_FONTS_REPO_DIR");
    println!("cargo:rerun-if-env-changed=EGUI_OFL_EXTRA_OFL_DIRS");
    println!("cargo:rerun-if-env-changed=GITHUB_TOKEN");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let use_google_api = feature_enabled("source-google-fonts");
    let use_local_fallback = feature_enabled("source-local-fallback");
    let use_submodule = feature_enabled("source-submodule");

    if !use_google_api && !use_local_fallback && !use_submodule {
        return Err("enable at least one source: `source-google-fonts`, `source-local-fallback`, or `source-submodule`".into());
    }

    let repo_dir = if use_submodule {
        resolve_google_fonts_repo_dir(&crate_root)?
    } else {
        None
    };

    if use_submodule && repo_dir.is_none() && !use_google_api && !use_local_fallback {
        return Err("`source-submodule` is enabled, but `third_party/google-fonts` was not found and auto-init failed; set GOOGLE_FONTS_REPO_DIR or enable another source feature".into());
    }

    let has_any_explicit_family_feature = font_manifest::FONT_FAMILIES
        .iter()
        .any(|family| feature_enabled(&family_feature_name(family.local_family_dir)));
    let select_all_families = feature_enabled("font-all") || !has_any_explicit_family_feature;

    let mut selected_families: Vec<BuildFamilySpec> = font_manifest::FONT_FAMILIES
        .iter()
        .filter(|family| {
            license_feature_enabled(family.license)
                && (select_all_families
                    || feature_enabled(&family_feature_name(family.local_family_dir)))
        })
        .map(build_family_from_manifest)
        .collect();

    if !license_feature_enabled(LicenseKind::Ofl) {
        let extras = extra_ofl_dirs();
        if !extras.is_empty() {
            return Err("EGUI_OFL_EXTRA_OFL_DIRS requires `license-ofl` feature".into());
        }
    }

    let existing_dirs: HashSet<String> = selected_families
        .iter()
        .map(|family| family.google_ofl_dir.clone())
        .collect();

    for extra_dir in extra_ofl_dirs() {
        if existing_dirs.contains(&extra_dir) {
            continue;
        }
        selected_families.push(build_family_from_extra_dir(&extra_dir));
    }

    if selected_families.is_empty() {
        return Err("no font family selected: enable one or more `font-*` features or set EGUI_OFL_EXTRA_OFL_DIRS".into());
    }

    let mut resolved_families: Vec<ResolvedFamily> = Vec::new();
    let mut stats = AcquisitionStats::default();

    for family in &selected_families {
        let face_files = discover_family_face_files(
            &family.google_ofl_dir,
            &family.local_family_dir,
            &crate_root,
            repo_dir.as_deref(),
            use_google_api,
            use_local_fallback,
        )?;

        if face_files.is_empty() {
            return Err(format!("no font faces discovered for family {}", family.display_name).into());
        }

        let mut faces = Vec::new();
        for file_name in face_files {
            let egui_name = file_stem_to_name(&file_name)?;
            let spec = AssetSpec {
                url: format!(
                    "https://raw.githubusercontent.com/google/fonts/main/ofl/{}/{}",
                    family.google_ofl_dir, file_name
                ),
                out_name: file_name.clone(),
                fallback_rel_path: format!("fonts/{}/{}", family.local_family_dir, file_name),
                repo_rel_path: format!("ofl/{}/{}", family.google_ofl_dir, file_name),
            };
            let source = ensure_asset(
                &spec,
                &crate_root,
                &out_dir,
                repo_dir.as_deref(),
                use_google_api,
                use_local_fallback,
            )?;
            record_asset_source(&mut stats, source);
            faces.push(ResolvedFace { egui_name, file_name });
        }

        let license_name = format!("OFL-{}.txt", family.local_family_dir);
        let license_spec = AssetSpec {
            url: format!(
                "https://raw.githubusercontent.com/google/fonts/main/ofl/{}/OFL.txt",
                family.google_ofl_dir
            ),
            out_name: license_name.clone(),
            fallback_rel_path: format!("licenses/{license_name}"),
            repo_rel_path: format!("ofl/{}/OFL.txt", family.google_ofl_dir),
        };
        let source = ensure_asset(
            &license_spec,
            &crate_root,
            &out_dir,
            repo_dir.as_deref(),
            use_google_api,
            use_local_fallback,
        )?;
        record_asset_source(&mut stats, source);

        resolved_families.push(ResolvedFamily {
            display_name: family.display_name.clone(),
            specimen_url: family.specimen_url.clone(),
            repo_url: family.repo_url.clone(),
            faces,
        });
    }

    let generated = out_dir.join("generated_fonts.rs");
    let source = generate_source(&resolved_families, &stats);
    fs::write(generated, source)?;

    println!(
        "cargo:warning=egui-ofl-fonts assets: total={}, cache={}, submodule={}, local-fallback={}, api={}",
        stats.total,
        stats.from_cache,
        stats.from_submodule,
        stats.from_local_fallback,
        stats.from_api,
    );

    Ok(())
}

fn resolve_google_fonts_repo_dir(crate_root: &Path) -> Result<Option<PathBuf>, Box<dyn Error>> {
    if let Some(path) = env::var_os("GOOGLE_FONTS_REPO_DIR") {
        return Ok(Some(PathBuf::from(path)));
    }

    let default_repo = crate_root.join("third_party/google-fonts");
    if !default_repo.exists() {
        try_init_google_fonts_submodule(crate_root)?;
    }

    if default_repo.exists() {
        Ok(Some(default_repo))
    } else {
        Ok(None)
    }
}

fn try_init_google_fonts_submodule(crate_root: &Path) -> Result<(), Box<dyn Error>> {
    let gitmodules = crate_root.join(".gitmodules");
    if !gitmodules.exists() {
        return Ok(());
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(crate_root)
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .arg("--")
        .arg("third_party/google-fonts")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!(
                "cargo:warning=egui-ofl-fonts: initialized submodule third_party/google-fonts (source-submodule)"
            );
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!(
                "cargo:warning=egui-ofl-fonts: could not init submodule third_party/google-fonts: {}",
                stderr.trim()
            );
        }
        Err(err) => {
            println!(
                "cargo:warning=egui-ofl-fonts: could not run git to init submodule third_party/google-fonts: {}",
                err
            );
        }
    }

    Ok(())
}

fn build_family_from_manifest(family: &FontFamilySpec) -> BuildFamilySpec {
    BuildFamilySpec {
        display_name: family.display_name.to_owned(),
        google_ofl_dir: family.google_ofl_dir.to_owned(),
        local_family_dir: family.local_family_dir.to_owned(),
        specimen_url: family.specimen_url.to_owned(),
        repo_url: family.repo_url.to_owned(),
    }
}

fn build_family_from_extra_dir(ofl_dir: &str) -> BuildFamilySpec {
    BuildFamilySpec {
        display_name: ofl_dir.to_owned(),
        google_ofl_dir: ofl_dir.to_owned(),
        local_family_dir: ofl_dir.to_owned(),
        specimen_url: format!("https://fonts.google.com/?query={ofl_dir}"),
        repo_url: format!("https://github.com/google/fonts/tree/main/ofl/{ofl_dir}"),
    }
}

fn extra_ofl_dirs() -> Vec<String> {
    let Some(raw) = env::var_os("EGUI_OFL_EXTRA_OFL_DIRS") else {
        return Vec::new();
    };

    let mut dirs = raw
        .to_string_lossy()
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_ascii_lowercase())
        .collect::<Vec<_>>();
    dirs.sort();
    dirs.dedup();
    dirs
}

fn discover_family_face_files(
    google_ofl_dir: &str,
    local_family_dir: &str,
    crate_root: &Path,
    repo_dir: Option<&Path>,
    use_google_api: bool,
    use_local_fallback: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    if let Some(repo_dir) = repo_dir {
        let repo_family_dir = repo_dir.join("ofl").join(google_ofl_dir);
        let files = list_local_face_files(&repo_family_dir)?;
        if !files.is_empty() {
            return Ok(files);
        }
    }

    if use_local_fallback {
        let local_dir = crate_root.join("fonts").join(local_family_dir);
        let files = list_local_face_files(&local_dir)?;
        if !files.is_empty() {
            return Ok(files);
        }
    }

    if use_google_api {
        let files = list_remote_family_face_files(google_ofl_dir)?;
        if !files.is_empty() {
            return Ok(files);
        }
    }

    Err(format!("no usable faces found for family {}", google_ofl_dir).into())
}

fn list_remote_family_face_files(google_ofl_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/google/fonts/contents/ofl/{}",
        google_ofl_dir
    );

    let mut request = ureq::get(&url)
        .header("User-Agent", "egui-ofl-fonts-build")
        .header("Accept", "application/vnd.github+json");

    if let Some(token) = env::var_os("GITHUB_TOKEN") {
        let auth = format!("Bearer {}", token.to_string_lossy());
        request = request.header("Authorization", &auth);
    }

    let response = match request.call() {
        Ok(response) => response,
        Err(ureq::Error::StatusCode(code)) if code == 403 => {
            return Err(format!(
                "GitHub API returned 403 while listing ofl/{}; set GITHUB_TOKEN or use `source-submodule` feature",
                google_ofl_dir
            )
            .into())
        }
        Err(err) => return Err(err.into()),
    };
    if response.status() != 200 {
        return Err(format!("unexpected HTTP status {} for {}", response.status(), url).into());
    }

    let mut body = response.into_body();
    let text = body.read_to_string()?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    let items = value
        .as_array()
        .ok_or("github contents response is not an array")?;

    let mut files = Vec::new();
    for item in items {
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or_default();
        if is_font_file(name) {
            files.push(name.to_owned());
        }
    }

    files.sort();
    Ok(files)
}

fn list_local_face_files(local_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    if !local_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(local_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy().to_string();
        if is_font_file(&file_name) {
            files.push(file_name);
        }
    }

    files.sort();
    Ok(files)
}

fn is_font_file(name: &str) -> bool {
    name.ends_with(".ttf") || name.ends_with(".otf")
}

fn file_stem_to_name(file_name: &str) -> Result<String, Box<dyn Error>> {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("invalid font file name: {}", file_name))?;
    Ok(stem.to_owned())
}

fn ensure_asset(
    spec: &AssetSpec,
    crate_root: &Path,
    out_dir: &Path,
    repo_dir: Option<&Path>,
    use_google_api: bool,
    use_local_fallback: bool,
) -> Result<AssetSource, Box<dyn Error>> {
    let out_path = out_dir.join(&spec.out_name);

    if out_path.exists() {
        return Ok(AssetSource::Cache);
    }

    if let Some(repo_dir) = repo_dir {
        let src = repo_dir.join(&spec.repo_rel_path);
        if src.exists() {
            fs::copy(src, &out_path)?;
            return Ok(AssetSource::Submodule);
        }
    }

    if use_local_fallback {
        let fallback = crate_root.join(&spec.fallback_rel_path);
        if fallback.exists() {
            fs::copy(fallback, &out_path)?;
            return Ok(AssetSource::LocalFallback);
        }
    }

    if use_google_api {
        download_to_file(&spec.url, &out_path)?;
        return Ok(AssetSource::Api);
    }

    Err(format!(
        "asset unavailable (repo: {}, fallback: {}, remote: {})",
        spec.repo_rel_path, spec.fallback_rel_path, spec.url
    )
    .into())
}

fn record_asset_source(stats: &mut AcquisitionStats, source: AssetSource) {
    stats.total += 1;
    match source {
        AssetSource::Cache => stats.from_cache += 1,
        AssetSource::Submodule => stats.from_submodule += 1,
        AssetSource::LocalFallback => stats.from_local_fallback += 1,
        AssetSource::Api => stats.from_api += 1,
    }
}

fn download_to_file(url: &str, out_path: &Path) -> Result<(), Box<dyn Error>> {
    let response = match ureq::get(url).call() {
        Ok(response) => response,
        Err(ureq::Error::StatusCode(code)) if code == 403 => {
            return Err(
                "GitHub raw download returned 403; set GITHUB_TOKEN or use `source-submodule` feature"
                    .into(),
            )
        }
        Err(err) => return Err(err.into()),
    };
    if response.status() != 200 {
        return Err(format!("unexpected HTTP status {} for {}", response.status(), url).into());
    }

    let mut body = response.into_body();
    let bytes = body.read_to_vec()?;
    fs::write(out_path, bytes)?;
    Ok(())
}

fn feature_enabled(feature_name: &str) -> bool {
    let env_name = format!(
        "CARGO_FEATURE_{}",
        feature_name.replace('-', "_").to_uppercase()
    );
    env::var_os(env_name).is_some()
}

fn family_feature_name(local_family_dir: &str) -> String {
    let slug = local_family_dir
        .chars()
        .map(|ch| match ch {
            '_' | ' ' => '-',
            _ => ch.to_ascii_lowercase(),
        })
        .collect::<String>();
    format!("font-{slug}")
}

fn license_feature_enabled(license: LicenseKind) -> bool {
    match license {
        LicenseKind::Ofl => feature_enabled("license-ofl"),
    }
}

fn generate_source(resolved_families: &[ResolvedFamily], stats: &AcquisitionStats) -> String {
    let mut source = String::new();

    source.push_str("pub const EMBEDDED_FONTS: &[EmbeddedFontEntry] = &[\n");
    for family in resolved_families {
        for face in &family.faces {
            source.push_str(&format!(
                "    EmbeddedFontEntry {{ name: {:?}, bytes: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{}\")) }},\n",
                face.egui_name, face.file_name
            ));
        }
    }
    source.push_str("];\n\n");

    source.push_str("pub const DEFAULT_PROPORTIONAL_ORDER: &[&str] = &[\n");
    for family in resolved_families {
        for face in &family.faces {
            source.push_str(&format!("    {:?},\n", face.egui_name));
        }
    }
    source.push_str("];\n\n");

    source.push_str("pub const EMBEDDED_FAMILY_INFO: &[EmbeddedFamilyInfo] = &[\n");
    for family in resolved_families {
        source.push_str(&format!(
            "    EmbeddedFamilyInfo {{ display_name: {:?}, specimen_url: {:?}, repo_url: {:?} }},\n",
            family.display_name, family.specimen_url, family.repo_url
        ));
    }
    source.push_str("];\n");

    source.push_str(&format!(
        "\npub const BUILD_ACQUISITION_SUMMARY: AssetAcquisitionSummary = AssetAcquisitionSummary {{ total: {}, cache: {}, submodule: {}, local_fallback: {}, api: {} }};\n",
        stats.total,
        stats.from_cache,
        stats.from_submodule,
        stats.from_local_fallback,
        stats.from_api
    ));

    source
}
