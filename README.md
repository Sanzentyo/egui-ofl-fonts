# egui-ofl-fonts

Small helper crate for loading OFL fonts into `egui`.

## Default behavior

With default features (no extra setup):

- API source is enabled (`source-google-fonts`)
- `Kiwi Maru` and `Hachi Maru Pop` are enabled
- If matching local files exist under `fonts/` and `licenses/`, they are used first
- Missing files are fetched at build time and embedded

So, by default, only Kiwi + Hachi are embedded.

## Included default families

- `HachiMaruPop-Regular`
- `KiwiMaru-Regular`, `KiwiMaru-Medium`, `KiwiMaru-Light`

## Build source policy

Build-time source order:

1. optional submodule/clone source (`source-submodule` + `GOOGLE_FONTS_REPO_DIR` or `third_party/google-fonts`)
2. local fallback files in this crate (`source-local-fallback`)
3. GitHub API + raw download (`source-google-fonts`)

This means existing local font files are reused first, and network fetch is only used when needed.

## Features

- `license-ofl`: enable OFL families
- `source-google-fonts`: enable GitHub API + raw download path
- `source-submodule`: enable local `google/fonts` repository path
- `source-local-fallback`: enable `fonts/` + `licenses/` fallback path
- `font-kiwi-maru`: enable Kiwi Maru family
- `font-hachi-maru-pop`: enable Hachi Maru Pop family
- `font-all`: enable all families listed in `src/font_manifest.rs`

Default features:

- `license-ofl`
- `source-google-fonts`
- `source-local-fallback`
- `font-kiwi-maru`
- `font-hachi-maru-pop`

## Submodule path (optional)

Submodule is optional and used only when `source-submodule` is enabled.

```bash
git submodule add https://github.com/google/fonts third_party/google-fonts
git submodule update --init --recursive
cargo check --no-default-features --features license-ofl,source-submodule,font-all
```

You can also point to another local clone:

```bash
GOOGLE_FONTS_REPO_DIR=/path/to/google-fonts cargo check --no-default-features --features license-ofl,source-submodule,font-all
```

When `source-submodule` is enabled, `build.rs` also tries:

```bash
git submodule update --init --recursive -- third_party/google-fonts
```

if `third_party/google-fonts` is missing and `.gitmodules` exists.

## Using as a git dependency

`[package].repository` in `Cargo.toml` is metadata only.
Actual dependency source is controlled by the dependent project's `[dependencies]` entry.

Example:

```toml
[dependencies]
egui-ofl-fonts = { git = "https://github.com/<owner>/<repo>", default-features = false, features = ["license-ofl", "source-submodule", "font-all"] }
```

Recommended options for consumers:

- submodule-first (with fallback): `license-ofl,source-submodule,source-local-fallback,font-all`
- network-first (no submodule needed): `license-ofl,source-google-fonts,font-all`

## Extra families beyond Kiwi/Hachi

You can include other OFL families from `google/fonts/ofl/*` by setting:

- `EGUI_OFL_EXTRA_OFL_DIRS` (comma-separated OFL directory names)

Example:

```bash
EGUI_OFL_EXTRA_OFL_DIRS="rocknrollone,zenmarugothic" cargo check
```

When using submodule source only:

```bash
git -C third_party/google-fonts sparse-checkout add ofl/rocknrollone ofl/zenmarugothic
cargo check --no-default-features --features license-ofl,source-submodule,font-all
```

## Troubleshooting (HTTP 403)

If build fails with `403` while using `source-google-fonts`:

1. Provide a GitHub token:

```bash
GITHUB_TOKEN=your_token_here cargo check
```

2. Or switch to submodule source:

```bash
cargo check --no-default-features --features license-ofl,source-submodule,font-all
```

3. Or keep `source-local-fallback` enabled and provide local files under `fonts/` and `licenses/`.

Build script error messages now include this guidance when 403 occurs.

## Usage

```rust
let app = move |cc: &eframe::CreationContext<'_>| {
    egui_ofl_fonts::install_fonts(&cc.egui_ctx);
    Ok(Box::new(MyApp::default()))
};
```

### Select fonts by string names

```rust
egui_ofl_fonts::install_selected_fonts!(
    &cc.egui_ctx,
    "KiwiMaru-Regular",
    "HachiMaruPop-Regular",
)?;
```

### Select fonts by enum (typed)

```rust
use egui_ofl_fonts::BuiltinFont;

egui_ofl_fonts::install_builtin_fonts!(
    &cc.egui_ctx,
    BuiltinFont::KiwiMaruRegular,
    BuiltinFont::HachiMaruPopRegular,
)?;
```

Run example programs:

```bash
cargo run --example install_default
cargo run --example install_selected
cargo run --example install_builtin
cargo run --example font_picker_gui
```

`font_picker_gui` also shows build-time source summary (cache/submodule/local/api).

LLM/automation-oriented operational notes are in `README.llm.md`.

## Runtime info

```rust
for family in egui_ofl_fonts::embedded_family_info() {
    println!("{} -> {}", family.display_name, family.repo_url);
}
```

## License

- SPDX: `MIT OR Apache-2.0`
- Rust source code in this crate: MIT OR Apache-2.0 (see `LICENSE-MIT` and `LICENSE-APACHE`)
- Bundled font files: SIL Open Font License 1.1
  - Hachi Maru Pop: `licenses/OFL-Hachi_Maru_Pop.txt`
  - Kiwi Maru: `licenses/OFL-Kiwi_Maru.txt`
