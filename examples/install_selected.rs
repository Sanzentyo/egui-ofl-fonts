fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = egui::Context::default();

    egui_ofl_fonts::install_selected_fonts!(
        &ctx,
        "KiwiMaru-Regular",
        "HachiMaruPop-Regular",
    )?;

    println!("selected fonts installed successfully");
    Ok(())
}
