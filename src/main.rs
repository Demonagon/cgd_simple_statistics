#![feature(slice_pattern)]

mod math;
mod stats;
mod ui;

use crate::{
    math::{cache::DataCache, enumeration::all_rule_indexes},
    stats::{StatisticalParameters, Statistics},
    ui::SimpleCGDStatisticsApp,
};

fn main() {
    let native_options = eframe::NativeOptions::default();

    let params: StatisticalParameters = StatisticalParameters {
        growth: stats::growth::GrowthParameters {
            configuration_count: 32,
            configurations_size: 32,
            simulation_steps: 32,
        },
    };

    let mut cache = DataCache::default();

    let alphabet_size = 3;
    let max_radius = 3;

    cache.precompute_up_to(alphabet_size, max_radius);

    let mut stats = Statistics::default();

    for radius in 0..=max_radius {
        let indexes = all_rule_indexes(&cache, alphabet_size, radius)
            .take(100000)
            .collect::<Vec<_>>();

        params.compute(&mut cache, alphabet_size, radius, &indexes, &mut stats);
    }

    println!("stats : {stats:?}");

    _ = eframe::run_native(
        "Simple CGD Statistics",
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    );
}
