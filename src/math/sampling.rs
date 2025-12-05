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

fn log(s: impl ToString) {
    if !cfg!(test) {
        println!("{}", s.to_string());
    }
}

impl SamplingDataCache {
    pub fn new(alphabet_size: u32, radius: u32) -> Self {
        // the max word size for a given radius is n + 1
        log("Computing sums...");
        let mut sums = SumsCache::default();
        sums.precompute_up_to(radius);
        log("done.");

        let mut words = vec![];

        log("Computing words...");
        for r in 0..=radius {
            log(format!("size {r}"));
            words.extend(self_bifix_free_words(alphabet_size, r + 1));
            log("done.");
        }

        //result.bifix_free_graph = compute_words_graph(&words);
        log("Computing graph...");
        let bifix_free_graph = DoubleTree::new(alphabet_size as usize, words).into_graph();
        log("done.");
        log("Computing ranges...");
        let graph_ranges = bifix_free_graph.word_size_ranges();
        log("done.");

        // we do a step to exclude sums from result.sums when it contains a radius
        // the word size of which has no initial index in result.graph_ranges

        log("Computing filtering sums...");
        for sum in sums.sums.values_mut() {
            sum.retain(|values| values.iter().all(|k| graph_ranges.contains_key(k)));
        }
        log("done.");

        Self {
            bifix_free_graph,
            graph_ranges,
            sums,
        }
    }

    pub fn new_number_conserving(alphabet_size: u32, radius: u32) -> Self {
        // the max word size for a given radius is n + 1
        log("Computing sums...");
        let mut sums = SumsCache::default();
        sums.precompute_up_to(radius);
        log("done.");

        let mut words = vec![];

        log("Computing words...");
        for r in 0..=radius {
            log("size {r}");
            words.extend(self_bifix_free_words(alphabet_size, r + 1));
            log("done.");
        }

        let mut sorted_words: HashMap<u32, Vec<Word>> = Default::default();

        for word in words {
            let sum = word.value_sum();
            sorted_words.entry(sum).or_default().push(word);
        }

        let mut bifix_free_graph = Graph::default();

        for (sum, words) in sorted_words {
            log(format!("Computing graph for sum {sum}..."));
            let graph = DoubleTree::new(alphabet_size as usize, words).into_graph();
            bifix_free_graph.append(graph);
            log("done.");
        }

        log("Computing ranges...");
        let graph_ranges = bifix_free_graph.word_size_ranges();
        log("done.");

        // we do a step to exclude sums from result.sums when it contains a radius
        // the word size of which has no initial index in result.graph_ranges

        log("Computing filtering sums...");
        for sum in sums.sums.values_mut() {
            sum.retain(|values| values.iter().all(|k| graph_ranges.contains_key(k)));
        }
        log("done.");

        Self {
            bifix_free_graph,
            graph_ranges,
            sums,
        }
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
