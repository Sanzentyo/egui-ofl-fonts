use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui-ofl-fonts: Font Picker",
        options,
        Box::new(|cc| {
            egui_ofl_fonts::install_fonts(&cc.egui_ctx);
            Ok(Box::<FontPickerApp>::default())
        }),
    )
}

struct FontPickerApp {
    fonts: Vec<String>,
    selected_font: String,
    preview_text: String,
    acquisition: egui_ofl_fonts::AssetAcquisitionSummary,
}

impl Default for FontPickerApp {
    fn default() -> Self {
        let mut fonts = egui_ofl_fonts::embedded_font_names()
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        fonts.sort();

        let selected_font = fonts
            .first()
            .cloned()
            .unwrap_or_else(|| "Proportional".to_owned());

        Self {
            fonts,
            selected_font,
            preview_text: "日本語フォントプレビュー: こんにちは、世界！\nThe quick brown fox jumps over the lazy dog.\n1234567890".to_owned(),
            acquisition: egui_ofl_fonts::build_acquisition_summary(),
        }
    }
}

impl eframe::App for FontPickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("egui-ofl-fonts Font Picker");
            ui.label(format!("Embedded fonts: {}", self.fonts.len()));
            ui.label(format!(
                "Build sources => total: {}, cache: {}, submodule: {}, local fallback: {}, api: {}",
                self.acquisition.total,
                self.acquisition.cache,
                self.acquisition.submodule,
                self.acquisition.local_fallback,
                self.acquisition.api,
            ));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Font:");
                egui::ComboBox::from_id_salt("font_combo")
                    .selected_text(&self.selected_font)
                    .show_ui(ui, |ui| {
                        for font in &self.fonts {
                            ui.selectable_value(&mut self.selected_font, font.clone(), font);
                        }
                    });
            });

            ui.separator();
            ui.label("Preview text:");
            ui.add(
                egui::TextEdit::multiline(&mut self.preview_text)
                    .desired_rows(4)
                    .desired_width(f32::INFINITY),
            );

            ui.separator();
            ui.label("Rendered preview:");
            ui.label(
                egui::RichText::new(&self.preview_text).font(egui::FontId::new(
                    28.0,
                    egui::FontFamily::Name(self.selected_font.clone().into()),
                )),
            );
        });
    }
}
