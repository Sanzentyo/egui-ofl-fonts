use egui::{FontData, FontDefinitions, FontFamily};
use std::fmt;
use std::sync::Arc;

mod font_manifest;
mod font_presets;

pub use font_presets::BuiltinFont;

pub struct EmbeddedFontEntry {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

pub struct EmbeddedFamilyInfo {
    pub display_name: &'static str,
    pub specimen_url: &'static str,
    pub repo_url: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AssetAcquisitionSummary {
    pub total: usize,
    pub cache: usize,
    pub submodule: usize,
    pub local_fallback: usize,
    pub api: usize,
}

include!(concat!(env!("OUT_DIR"), "/generated_fonts.rs"));

#[derive(Debug, Clone)]
pub struct FontSelectionError {
    pub missing_fonts: Vec<String>,
}

impl fmt::Display for FontSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "requested fonts are not embedded: {}", self.missing_fonts.join(", "))
    }
}

impl std::error::Error for FontSelectionError {}

pub fn embedded_family_info() -> &'static [EmbeddedFamilyInfo] {
    EMBEDDED_FAMILY_INFO
}

pub fn build_acquisition_summary() -> AssetAcquisitionSummary {
    BUILD_ACQUISITION_SUMMARY
}

pub fn embedded_font_names() -> Vec<&'static str> {
    EMBEDDED_FONTS.iter().map(|entry| entry.name).collect()
}

pub fn ofl_font_definitions() -> FontDefinitions {
    ofl_font_definitions_with(DEFAULT_PROPORTIONAL_ORDER)
        .expect("default embedded font order must contain only embedded fonts")
}

pub fn ofl_font_definitions_with(selected_names: &[&str]) -> Result<FontDefinitions, FontSelectionError> {
    let mut missing_fonts = Vec::new();
    let mut selected_entries = Vec::new();

    for selected in selected_names {
        if let Some(entry) = EMBEDDED_FONTS.iter().find(|entry| entry.name == *selected) {
            selected_entries.push(entry);
        } else {
            missing_fonts.push((*selected).to_owned());
        }
    }

    if !missing_fonts.is_empty() {
        return Err(FontSelectionError { missing_fonts });
    }

    let mut fonts = FontDefinitions::default();

    for embedded in &selected_entries {
        fonts.font_data.insert(
            embedded.name.to_owned(),
            Arc::new(FontData::from_static(embedded.bytes)),
        );
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        for (idx, name) in selected_names.iter().enumerate() {
            family.insert(idx, (*name).to_owned());
        }
    }

    for embedded in &selected_entries {
        fonts.families.insert(
            FontFamily::Name(Arc::<str>::from(embedded.name)),
            vec![embedded.name.to_owned()],
        );
    }

    Ok(fonts)
}

pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(ofl_font_definitions());
}

pub fn install_fonts_with(ctx: &egui::Context, selected_names: &[&str]) -> Result<(), FontSelectionError> {
    let fonts = ofl_font_definitions_with(selected_names)?;
    ctx.set_fonts(fonts);
    Ok(())
}

pub fn install_builtin_fonts(
    ctx: &egui::Context,
    selected_fonts: &[BuiltinFont],
) -> Result<(), FontSelectionError> {
    let selected_names = selected_fonts
        .iter()
        .copied()
        .map(BuiltinFont::as_name)
        .collect::<Vec<_>>();
    install_fonts_with(ctx, &selected_names)
}

#[macro_export]
macro_rules! install_selected_fonts {
    ($ctx:expr, $($font_name:expr),+ $(,)?) => {
        $crate::install_fonts_with($ctx, &[$($font_name),+])
    };
}

#[macro_export]
macro_rules! install_builtin_fonts {
    ($ctx:expr, $($font:expr),+ $(,)?) => {
        $crate::install_builtin_fonts($ctx, &[$($font),+])
    };
}
