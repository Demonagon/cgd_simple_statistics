use crate::{
    math::{
        cache::DataCache,
        enumeration::{number_conserving_rule_from_index, rule_from_index},
        permutation::rules_radius,
        sampling::SamplingDataCache,
        shift::generate_random_shift,
    },
    stats::growth::{
        GrowthClassification, GrowthData, GrowthParameters, is_rule_constant, is_rule_growing,
    },
};

pub mod growth;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Default, Clone)]
pub struct Statistics {
    pub growth: GrowthData,
}

#[derive(Debug, Clone)]
pub struct StatisticalParameters {
    pub growth: GrowthParameters,
}

impl StatisticalParameters {
    pub fn compute(
        &self,
        cache: &mut DataCache,
        alphabet_size: u32,
        radius: u32,
        indexes: &[u32],
        statistics: &mut Statistics,
    ) {
        // we create an iterator that goes in parallel
        // for each rule, we do checks depending on the statistics
        // growth statistics run X configuration of size Y, for Z steps, and if one of them
        // goes 2x on size, the rule is considered growing

        let is_growing = indexes
            .into_par_iter()
            .map(|k| rule_from_index(cache, alphabet_size, radius, *k))
            .map(|r| {
                let mut rng = rand::rng();
                let rule_radius = rules_radius(&r);
                let classification = if is_rule_constant(&r) {
                    GrowthClassification::Constant
                } else if is_rule_growing(&self.growth, &mut rng, alphabet_size, &r) {
                    GrowthClassification::Growing
                } else {
                    GrowthClassification::NonGrowing
                };

                (rule_radius, classification)
            })
            .collect::<Vec<_>>();

        for (radius, class) in is_growing {
            statistics.growth.insert(alphabet_size, radius, class);
        }
    }

    pub fn sample(
        &self,
        cache: &SamplingDataCache,
        alphabet_size: u32,
        radius: u32,
        count: u32,
        statistics: &mut Statistics,
    ) {
        // we create an iterator that goes in parallel
        // for each rule, we do checks depending on the statistics
        // growth statistics run X configuration of size Y, for Z steps, and if one of them
        // goes 2x on size, the rule is considered growing

        let is_growing = (0..count)
            .into_par_iter()
            .progress()
            .map(|_| {
                let mut rng = rand::rng();

                let rule = cache.sample_rule(radius, &mut rng);

                let rule_radius = rules_radius(&rule);

                let classification = if is_rule_constant(&rule) {
                    GrowthClassification::Constant
                } else if is_rule_growing(&self.growth, &mut rng, alphabet_size, &rule) {
                    GrowthClassification::Growing
                } else {
                    GrowthClassification::NonGrowing
                };

                (rule_radius, classification)
            })
            .collect::<Vec<_>>();

        for (radius, class) in is_growing {
            statistics.growth.insert(alphabet_size, radius, class);
        }
    }

    pub fn sample_shift(
        &self,
        cache: &SamplingDataCache,
        alphabet_size: u32,
        radius: u32,
        count: u32,
        statistics: &mut Statistics,
    ) {
        // we create an iterator that goes in parallel
        // for each rule, we do checks depending on the statistics
        // growth statistics run X configuration of size Y, for Z steps, and if one of them
        // goes 2x on size, the rule is considered growing

        let is_growing = (0..count)
            .into_par_iter()
            .progress()
            .map(|_| {
                let mut rng = rand::rng();

                // we ask for a simple rule set
                let rule = cache.sample_rule(radius - 1, &mut rng);

                // to compensate with the +1 of the shift
                let rule_radius = rules_radius(&rule) + 1;

                let shift = generate_random_shift(alphabet_size, &mut rng);

                let mut shift_rule = rule.clone();
                shift_rule.extend(shift);

                let classification = if is_rule_constant(&rule) {
                    GrowthClassification::Constant
                } else if is_rule_growing(&self.growth, &mut rng, alphabet_size, &shift_rule) {
                    GrowthClassification::Growing
                } else {
                    GrowthClassification::NonGrowing
                };

                (rule_radius, classification)
            })
            .collect::<Vec<_>>();

        for (radius, class) in is_growing {
            statistics.growth.insert(alphabet_size, radius, class);
        }
    }

    pub fn compute_number_conserving(
        &self,
        cache: &mut DataCache,
        alphabet_size: u32,
        radius: u32,
        indexes: &[u32],
        statistics: &mut Statistics,
    ) {
        // we create an iterator that goes in parallel
        // for each rule, we do checks depending on the statistics
        // growth statistics run X configuration of size Y, for Z steps, and if one of them
        // goes 2x on size, the rule is considered growing

        let is_growing = indexes
            .into_par_iter()
            .map(|k| number_conserving_rule_from_index(cache, alphabet_size, radius, *k))
            .map(|r| {
                let mut rng = rand::rng();
                let rule_radius = rules_radius(&r);
                let classification = if is_rule_constant(&r) {
                    GrowthClassification::Constant
                } else if is_rule_growing(&self.growth, &mut rng, alphabet_size, &r) {
                    GrowthClassification::Growing
                } else {
                    GrowthClassification::NonGrowing
                };

                (rule_radius, classification)
            })
            .collect::<Vec<_>>();

        for (radius, class) in is_growing {
            statistics.growth.insert(alphabet_size, radius, class);
        }
    }
}
