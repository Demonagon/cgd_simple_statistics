use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eframe::egui::{self, Button, DragValue, ProgressBar};
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Default)]
pub struct SimpleCGDStatisticsApp {
    progress: Arc<Mutex<Option<f32>>>,
    alphabet_size: u32,
    radius: u32,
}

impl eframe::App for SimpleCGDStatisticsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(progress) = *self.progress.lock().unwrap() {
            egui::Window::new("Loading...").show(ctx, |ui| {
                ui.add(ProgressBar::new(progress).show_percentage().animate(true));
            });
        }

        egui::SidePanel::left("Generate").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("alphabet size:");
                ui.add(DragValue::new(&mut self.alphabet_size).range(2..=16));
            });

            ui.horizontal(|ui| {
                ui.label("radius:");
                ui.add(DragValue::new(&mut self.radius).range(0..=128));
            });

            let button_active = self.progress.lock().unwrap().is_none();

            if ui
                .add(Button::selectable(button_active, "Click me"))
                .clicked()
                && button_active
            {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            let sin: PlotPoints = (0..1000)
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                })
                .collect();
            let line = Line::new("sin", sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}
