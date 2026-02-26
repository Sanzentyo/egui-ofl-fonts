use egui_ofl_fonts::BuiltinFont;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = egui::Context::default();

    egui_ofl_fonts::install_builtin_fonts!(
        &ctx,
        BuiltinFont::KiwiMaruRegular,
        BuiltinFont::HachiMaruPopRegular,
    )?;

    println!("builtin enum fonts installed successfully");
    Ok(())
}
