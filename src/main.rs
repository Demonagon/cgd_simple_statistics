mod math;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();

    _ = eframe::run_native(
        "Simple CGD Statistics",
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    );
}

#[derive(Default)]
pub struct SimpleCGDStatisticsApp {
    is_process_ongoing: Arc<Mutex<bool>>,
}

impl eframe::App for SimpleCGDStatisticsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("'sup");

            if !*self.is_process_ongoing.lock().unwrap() {
                if ui.button("Click me").clicked() {
                    *self.is_process_ongoing.lock().unwrap() = true;

                    let copy = Arc::clone(&self.is_process_ongoing);

                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_secs(3));
                        *copy.lock().unwrap() = false;
                    });
                }
            } else {
                ui.spinner();
            }
        });
    }
}
