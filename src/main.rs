#![feature(slice_pattern)]
#![allow(dead_code)]

mod math;
mod stats;
mod ui;

use crate::{
    math::sampling::SamplingDataCache,
    stats::{StatisticalParameters, Statistics},
};

fn main() {
    // let native_options = eframe::NativeOptions::default();

    let params: StatisticalParameters = StatisticalParameters {
        growth: stats::growth::GrowthParameters {
            configuration_count: 32,
            configurations_size: 32,
            simulation_steps: 32,
        },
    };

    let alphabet_size = 3;
    let max_radius = 7;

    let sample_cache = SamplingDataCache::new(alphabet_size, max_radius);

    // let mut rng = rand::rng();
    // for _k in 0..10 {
    //     println!("=============");
    //     for perm in sample_cache.sample_rule(max_radius, &mut rng) {
    //         println!("[{perm}]");
    //     }
    // }
    // println!("=============");

    // let mut cache = DataCache::default();

    // cache.precompute_up_to(alphabet_size, max_radius);

    let mut stats = Statistics::default();

    for radius in 0..=max_radius {
        //     let indexes = all_rule_indexes(&cache, alphabet_size, radius)
        //         .take(100000)
        //         .collect::<Vec<_>>();

        //     params.compute(&mut cache, alphabet_size, radius, &indexes, &mut stats);
        params.sample(&sample_cache, alphabet_size, radius, 10000, &mut stats);
    }

    let mut data = stats.growth.data.into_iter().collect::<Vec<_>>();
    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (radius, point) in data {
        println!("alphabet_size, radius {radius:?} :");
        println!("{point:?}");
        println!("{}%", point.growing_proportion().unwrap_or(0.0) * 100.0);
    }

    // _ = eframe::run_native(
    //     "Simple CGD Statistics",
    //     native_options,
    //     Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    // );
}
