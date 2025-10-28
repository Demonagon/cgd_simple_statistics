mod math;
mod stats;
mod ui;

use crate::{
    math::{cache::DataCache, enumeration::all_rule_indexes},
    stats::StatisticalParameters,
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
    let radius = 2;

    cache.precompute_up_to(alphabet_size, radius);

    let indexes = all_rule_indexes(&cache, alphabet_size, radius)
        .take(100000)
        .collect::<Vec<_>>();

    let stats = params.compute(&mut cache, alphabet_size, radius, &indexes);

    println!("stats : {stats:?}");

    _ = eframe::run_native(
        "Simple CGD Statistics",
        native_options,
        Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    );
}
