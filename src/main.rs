mod math;
mod stats;
mod ui;

use crate::ui::SimpleCGDStatisticsApp;

fn main() {
    let native_options = eframe::NativeOptions::default();

    _ = eframe::run_native(
        "Simple CGD Statistics",
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    );
}
