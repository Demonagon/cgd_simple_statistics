use std::collections::HashMap;

use crate::math::{
    permutation::{Permutation, apply_rules},
    word::Word,
};

use rand::Rng;

#[derive(Copy, Clone, Default, Debug)]
pub struct GrowthDataPoint {
    pub growing: u32,
    pub non_growing: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct GrowthParameters {
    pub configuration_count: u32,
    pub configurations_size: u32,
    pub simulation_steps: u32,
}

impl GrowthDataPoint {
    pub fn growing_proportion(&self) -> Option<f32> {
        if self.non_growing + self.growing == 0 {
            return None;
        }

        Some(self.growing as f32 / (self.non_growing as f32 + self.growing as f32))
    }
}

#[derive(Default)]
pub struct GrowthData {
    // order : alphabet_size, radius
    data: HashMap<(u32, u32), GrowthDataPoint>,
}

impl GrowthData {
    pub fn insert(&mut self, alphabet_size: u32, radius: u32, growing: bool) {
        let data_point = self.data.entry((alphabet_size, radius)).or_default();
        match growing {
            true => data_point.growing += 1,
            false => data_point.non_growing += 1,
        };
    }

    pub fn extract(&self, alphabet_size: u32, radius: u32) -> Option<f32> {
        if let Some(data_point) = self.data.get(&(alphabet_size, radius)) {
            return data_point.growing_proportion();
        }

        None
    }
}

pub fn is_rule_growing<R: Rng>(
    parameters: &GrowthParameters,
    rng: &mut R,
    alphabet_size: u32,
    rules: &[Permutation],
) -> bool {
    // maybe run in parallel ? test

    (0..parameters.configuration_count)
        .map(|_| Word::random(rng, parameters.configurations_size, alphabet_size))
        .any(|mut w| {
            let initial_size = w.len();

            for _ in 0..parameters.simulation_steps {
                w = apply_rules(rules, w);
                if w.len() >= initial_size * 2 {
                    return true;
                }
            }

            false
        })
}
