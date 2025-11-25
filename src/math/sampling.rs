use std::collections::HashMap;

use rand::{Rng, seq::IndexedRandom};

use crate::math::{
    cache::SumsCache, derangement::sample_permutation, double_tree::DoubleTree, graph::Graph,
    permutation::Permutation, set::self_bifix_free_words, word::Word,
};

#[derive(Default, Debug)]
pub struct SamplingDataCache {
    pub bifix_free_graph: Graph<Word>,
    pub graph_ranges: HashMap<u32, Vec<usize>>,
    pub sums: SumsCache,
}

impl SamplingDataCache {
    pub fn new(alphabet_size: u32, radius: u32) -> Self {
        let mut result = Self::default();

        // the max word size for a given radius is n + 1
        println!("Computing sums...");
        result.sums.precompute_up_to(radius);
        println!("done.");

        let mut words = vec![];

        println!("Computing words...");
        for r in 0..=radius {
            println!("size {r}");
            words.extend(self_bifix_free_words(alphabet_size, r + 1));
            println!("done.");
        }

        //result.bifix_free_graph = compute_words_graph(&words);
        println!("Computing graph...");
        result.bifix_free_graph = DoubleTree::new(alphabet_size as usize, words).into_graph();
        println!("done.");
        println!("Computing ranges...");
        result.graph_ranges = result.bifix_free_graph.word_size_ranges();
        println!("done.");

        // we do a step to exclude sums from result.sums when it contains a radius
        // the word size of which has no initial index in result.graph_ranges

        println!("Computing filtering sums...");
        for sums in result.sums.sums.values_mut() {
            sums.retain(|values| values.iter().all(|k| result.graph_ranges.contains_key(k)));
        }
        println!("done.");

        //self.sums.sums.retain(|_, sums| !sums.is_empty());

        result
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
