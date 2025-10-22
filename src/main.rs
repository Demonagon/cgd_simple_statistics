mod math;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eframe::egui::{self, Color32, Frame, ProgressBar};

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
    progress: Arc<Mutex<Option<f32>>>,
}

impl eframe::App for SimpleCGDStatisticsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(progress) = *self.progress.lock().unwrap() {
            egui::Area::new(egui::Id::new("loading_popup"))
                .default_size([10.0, 10.0])
                .show(ctx, |ui| {
                    Frame::new().fill(Color32::BLACK).show(ui, |ui| {
                        ui.add(ProgressBar::new(progress).show_percentage().animate(true));
                    })
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("'sup");

            if let Some(progress) = *self.progress.lock().unwrap() {
                ui.add(ProgressBar::new(progress).show_percentage().animate(true));
            } else if ui.button("Click me").clicked() {
                *self.progress.lock().unwrap() = Some(0.0);

                let copy = Arc::clone(&self.progress);

                std::thread::spawn(move || {
                    for k in 1..=10 {
                        std::thread::sleep(Duration::from_secs(1));
                        *copy.lock().unwrap() = Some(k as f32 / 10.0);
                    }
                    *copy.lock().unwrap() = None;
                });
            }
        });
    }
}
