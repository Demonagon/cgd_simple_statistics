use std::collections::HashMap;

use rand::{Rng, seq::IndexedRandom};

use crate::math::{
    cache::SumsCache,
    derangement::sample_permutation,
    graph::Graph,
    permutation::Permutation,
    set::{compute_words_graph, self_bifix_free_words},
    word::Word,
};

#[derive(Default, Debug)]
pub struct SamplingDataCache {
    pub bifix_free_graph: Graph<Word>,
    pub graph_ranges: HashMap<u32, Vec<usize>>,
    pub sums: SumsCache,
}

impl SamplingDataCache {
    pub fn precompute_up_to(&mut self, alphabet_size: u32, radius: u32) {
        // the max word size for a given radius is n + 1
        self.sums.precompute_up_to(radius);

        let mut words = vec![];

        for r in 0..=radius {
            words.extend(self_bifix_free_words(alphabet_size, r + 1));
        }

        self.bifix_free_graph = compute_words_graph(&words);
        self.graph_ranges = self.bifix_free_graph.word_size_ranges();

        // we do a step to exclude sums from self.sums when it contains a radius
        // the word size of which has no initial index in self.graph_ranges

        for sums in self.sums.sums.values_mut() {
            sums.retain(|values| values.iter().all(|k| self.graph_ranges.contains_key(k)));
        }

        //self.sums.sums.retain(|_, sums| !sums.is_empty());
    }

    pub fn sample_rule(&self, radius: u32, rng: &mut impl Rng) -> Vec<Permutation> {
        let mut result = vec![];

        let radii = self
            .sums
            .sums
            .get(&radius)
            .expect("Precompute sums before sampling a rule");

        let Some(radii) = radii.choose(rng) else {
            return vec![];
        };

        for radius in radii {
            let possible_initial_indexes = self
                .graph_ranges
                .get(&(radius + 1))
                .expect("Graph doesn't have words large enough for required radius");

            let initial_index = possible_initial_indexes
                .choose(rng)
                .expect("no word starting a clique matches the required index");

            let clique = self.bifix_free_graph.sample_clique(*initial_index, rng);
            let set = clique
                .into_iter()
                .map(|k| self.bifix_free_graph.get(k).unwrap().clone())
                .collect::<Vec<_>>();

            let permutation = sample_permutation(set.len() as u32, rng);

            result.push(Permutation {
                set,
                map: permutation,
            });
        }

        result
    }
}
