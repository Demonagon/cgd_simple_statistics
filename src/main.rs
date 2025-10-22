mod math;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();

    _ = eframe::run_native(
        "Simple CGD Statistics",
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp))),
    );
}

#[derive(Default)]
pub struct SimpleCGDStatisticsApp;

impl eframe::App for SimpleCGDStatisticsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("'sup");
        });
    }
}
