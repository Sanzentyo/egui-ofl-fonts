use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

pub const KIWI_MARU_REGULAR: &str = "KiwiMaru-Regular";
pub const KIWI_MARU_MEDIUM: &str = "KiwiMaru-Medium";
pub const KIWI_MARU_LIGHT: &str = "KiwiMaru-Light";
pub const HACHI_MARU_POP_REGULAR: &str = "HachiMaruPop-Regular";

pub fn ofl_font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        KIWI_MARU_REGULAR.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../fonts/Kiwi_Maru/KiwiMaru-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        KIWI_MARU_MEDIUM.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../fonts/Kiwi_Maru/KiwiMaru-Medium.ttf"
        ))),
    );
    fonts.font_data.insert(
        KIWI_MARU_LIGHT.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../fonts/Kiwi_Maru/KiwiMaru-Light.ttf"
        ))),
    );
    fonts.font_data.insert(
        HACHI_MARU_POP_REGULAR.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../fonts/Hachi_Maru_Pop/HachiMaruPop-Regular.ttf"
        ))),
    );

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, KIWI_MARU_REGULAR.to_owned());
        family.insert(1, KIWI_MARU_MEDIUM.to_owned());
        family.insert(2, KIWI_MARU_LIGHT.to_owned());
        family.insert(3, HACHI_MARU_POP_REGULAR.to_owned());
    }

    fonts.families.insert(
        FontFamily::Name(Arc::<str>::from(KIWI_MARU_REGULAR)),
        vec![KIWI_MARU_REGULAR.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(Arc::<str>::from(KIWI_MARU_MEDIUM)),
        vec![KIWI_MARU_MEDIUM.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(Arc::<str>::from(KIWI_MARU_LIGHT)),
        vec![KIWI_MARU_LIGHT.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(Arc::<str>::from(HACHI_MARU_POP_REGULAR)),
        vec![HACHI_MARU_POP_REGULAR.to_owned()],
    );

    fonts
}

pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(ofl_font_definitions());
}
