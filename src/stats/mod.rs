use crate::{
    math::{cache::DataCache, enumeration::rule_from_index, permutation::rules_radius},
    stats::growth::{
        GrowthClassification, GrowthData, GrowthParameters, is_rule_constant, is_rule_growing,
    },
};

pub mod growth;

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
}
