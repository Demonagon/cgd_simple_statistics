use crate::{
    math::{cache::DataCache, enumeration::rule_from_index, permutation::rules_radius},
    stats::growth::{GrowthData, GrowthParameters, is_rule_growing},
};

pub mod growth;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Statistics {
    pub growth: GrowthData,
}

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
    ) -> Statistics {
        // we create an iterator that goes in parallel
        // for each rule, we do checks depending on the statistics
        // growth statistics run X configuration of size Y, for Z steps, and if one of them
        // goes 2x on size, the rule is considered growing

        cache.precompute_up_to(alphabet_size, radius);

        let is_growing = indexes
            .into_par_iter()
            .map(|k| rule_from_index(cache, alphabet_size, radius, *k))
            .map(|r| {
                let mut rng = rand::rng();
                let rule_radius = rules_radius(&r);
                (
                    rule_radius,
                    is_rule_growing(&self.growth, &mut rng, alphabet_size, &r),
                )
            })
            .collect::<Vec<_>>();

        let mut growth_statistics = GrowthData::default();

        for (radius, is_growing) in is_growing {
            growth_statistics.insert(alphabet_size, radius, is_growing);
        }

        Statistics {
            growth: growth_statistics,
        }
    }
}
