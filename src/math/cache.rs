use std::collections::HashMap;

use crate::math::{
    derangement::derangements, set::bifix_free_sets, sum::enumerate_sorted_sums, word::Word,
};

#[derive(Default)]
pub struct DerangementCache {
    data: HashMap<u32, Vec<Vec<usize>>>,
}

impl DerangementCache {
    fn precompute(&mut self, size: u32) {
        self.data.entry(size).or_insert_with(|| derangements(size));
    }

    pub fn precompute_up_to(&mut self, max_size: u32) {
        for size in 0..=max_size {
            self.precompute(size);
        }
    }

    pub fn get(&self, size: u32) -> Option<&Vec<Vec<usize>>> {
        self.data.get(&size)
    }
}

// Each entry contains the sets, of which their biggest word
// is of that length
#[derive(Default)]
pub struct BifixSetCache {
    // key: (alphabet_size, max_word_size)
    data: HashMap<(u32, u32), Vec<Vec<Word>>>,
    max_set_size: usize,
}

impl BifixSetCache {
    pub fn precompute(&mut self, alphabet_size: u32, max_word_size: u32) {
        // we clear the previous results
        self.data.clear();
        self.max_set_size = 0;

        let new_sets = bifix_free_sets(alphabet_size, max_word_size);

        for set in new_sets {
            let max_word_size = set.iter().map(|w| w.len() as u32).max().unwrap_or(0);

            let len = set.len();

            self.data
                .entry((alphabet_size, max_word_size))
                .or_default()
                .push(set);

            if len > self.max_set_size {
                self.max_set_size = len;
            }
        }
    }

    pub fn get(&self, alphabet_size: u32, max_word_size: u32) -> Option<&Vec<Vec<Word>>> {
        self.data.get(&(alphabet_size, max_word_size))
    }
}

#[derive(Default)]
pub struct SumsCache {
    pub sums: HashMap<u32, Vec<Vec<u32>>>,
}

impl SumsCache {
    pub fn precompute_up_to(&mut self, max_sum: u32) {
        (0..=max_sum).for_each(|k| {
            self.sums.entry(k).or_insert(enumerate_sorted_sums(k));
        });
    }
}

/// A structure that holds counts of things, for dynamics programming-based optimisations.
#[derive(Default)]
pub struct PossibilitiesCache {
    // counts how many permutations of one size we can do
    pub permutation_possibilities: HashMap<(u32, u32), u32>,
}

#[derive(Default)]
pub struct DataCache {
    pub derangements: DerangementCache,
    pub sets: BifixSetCache,
    pub sums: SumsCache,
    pub possibilities: PossibilitiesCache,
}

impl DataCache {
    fn precompute_possibilities(&mut self, alphabet_size: u32) {
        self.possibilities.permutation_possibilities.clear();

        for ((a_size, w_size), sets) in self.sets.data.iter() {
            if *a_size != alphabet_size {
                continue;
            }

            for set in sets {
                let possible_derangements =
                    self.derangements.get(set.len() as u32).unwrap().len() as u32;

                *self
                    .possibilities
                    .permutation_possibilities
                    .entry((*a_size, *w_size))
                    .or_insert(0) += possible_derangements;
            }
        }

        // we account for the identity, which as radius 0
        *self
            .possibilities
            .permutation_possibilities
            .entry((alphabet_size, 1))
            .or_insert(0) += 1;
    }

    pub fn precompute_up_to(&mut self, alphabet_size: u32, radius: u32) {
        // the max word size for a given radius is n + 1
        self.sets.precompute(alphabet_size, radius + 1);
        self.derangements
            .precompute_up_to(self.sets.max_set_size as u32);
        self.sums.precompute_up_to(radius);
        self.precompute_possibilities(alphabet_size);
    }
}
