fn main() {
    let ctx = egui::Context::default();

    egui_ofl_fonts::install_fonts(&ctx);

    let embedded = egui_ofl_fonts::embedded_font_names();
    println!("installed {} embedded fonts", embedded.len());
    for name in embedded {
        println!("- {}", name);
    }
}
