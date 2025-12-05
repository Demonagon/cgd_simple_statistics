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

    let parameters = [
        (4, 5), //(3, 9),
                //(4, 7), (5, 6), (6, 4), (7, 3), (8, 3)
    ];

    for (a, b) in parameters {
        compute(a, b);
    }

    // _ = eframe::run_native(
    //     "Simple CGD Statistics",
    //     native_options,
    //     Box::new(|_cc| Ok(Box::new(SimpleCGDStatisticsApp::default()))),
    // );
}

fn compute(alphabet_size: u32, max_radius: u32) {
    println!("alphabet {alphabet_size}, radius {max_radius}");
    println!("Computing cache...");

    let params: StatisticalParameters = StatisticalParameters {
        growth: stats::growth::GrowthParameters {
            configuration_count: 32,
            configurations_size: 32,
            simulation_steps: 128,
        },
    };

    let sample_cache = SamplingDataCache::new(alphabet_size, max_radius);

    // let mut rng = rand::rng();
    // for _k in 0..10 {
    //     println!("=============");
    //     for perm in sample_cache.sample_rule(max_radius, &mut rng) {
    //         println!("[{perm}]");
    //     }
    // }
    // println!("=============");

    // return;

    // let mut cache = DataCache::default();

    // cache.precompute_up_to(alphabet_size, max_radius);

    let mut stats = Statistics::default();

    for radius in 1..=max_radius {
        println!("radius {radius}..");
        //     let indexes = all_rule_indexes(&cache, alphabet_size, radius)
        //         .take(100000)
        //         .collect::<Vec<_>>();

        //     params.compute(&mut cache, alphabet_size, radius, &indexes, &mut stats);
        params.sample(&sample_cache, alphabet_size, radius, 5000, &mut stats);
    }

    let mut data = stats.growth.data.into_iter().collect::<Vec<_>>();
    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (radius, point) in data {
        println!("alphabet_size, radius {radius:?} :");
        println!("{point:?}");
        println!("{}%", point.growing_proportion().unwrap_or(0.0) * 100.0);
    }
}

fn compute_shift(alphabet_size: u32, max_radius: u32) {
    println!("alphabet {alphabet_size}, radius {max_radius}");
    println!("Computing cache...");

    let params: StatisticalParameters = StatisticalParameters {
        growth: stats::growth::GrowthParameters {
            configuration_count: 32,
            configurations_size: 32,
            simulation_steps: 128,
        },
    };

    let sample_cache = SamplingDataCache::new(alphabet_size, max_radius);

    // let mut rng = rand::rng();
    // for _k in 0..10 {
    //     println!("=============");
    //     for perm in sample_cache.sample_rule(max_radius, &mut rng) {
    //         println!("[{perm}]");
    //     }
    // }
    // println!("=============");

    // return;

    // let mut cache = DataCache::default();

    // cache.precompute_up_to(alphabet_size, max_radius);

    let mut stats = Statistics::default();

    for radius in 1..=max_radius {
        println!("radius {radius}..");
        //     let indexes = all_rule_indexes(&cache, alphabet_size, radius)
        //         .take(100000)
        //         .collect::<Vec<_>>();

        //     params.compute(&mut cache, alphabet_size, radius, &indexes, &mut stats);
        params.sample_shift(&sample_cache, alphabet_size, radius, 5000, &mut stats);
    }

    let mut data = stats.growth.data.into_iter().collect::<Vec<_>>();
    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (radius, point) in data {
        println!("alphabet_size, radius {radius:?} :");
        println!("{point:?}");
        println!("{}%", point.growing_proportion().unwrap_or(0.0) * 100.0);
    }
}
