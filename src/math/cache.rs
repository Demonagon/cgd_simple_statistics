use std::collections::HashMap;

use crate::math::{derangement::derangements, set::bifix_free_sets, word::Word};

#[derive(Default)]
pub struct DerangementCache {
    data: HashMap<u32, Vec<Vec<usize>>>,
}

impl DerangementCache {
    fn precompute(&mut self, size: u32) {
        let new_maps = self
            .data
            .entry(size)
            .or_insert_with(|| derangements(size).collect::<Vec<_>>());
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

#[derive(Default)]
pub struct BifixSetCache {
    // key: (alphabet_size, max_word_size)
    data: HashMap<(u32, u32), Vec<Vec<Word>>>,
    max_set_size: usize,
}

impl BifixSetCache {
    pub fn precompute(&mut self, alphabet_size: u32, max_word_size: u32) {
        let new_sets = self
            .data
            .entry((alphabet_size, max_word_size))
            .or_insert(bifix_free_sets(alphabet_size, max_word_size));

        for set in new_sets {
            if set.len() > self.max_set_size {
                self.max_set_size = set.len();
            }
        }
    }

    pub fn get(&self, alphabet_size: u32, max_word_size: u32) -> Option<&Vec<Vec<Word>>> {
        self.data.get(&(alphabet_size, max_word_size))
    }
}

#[derive(Default)]
pub struct DataCache {
    pub derangements: DerangementCache,
    pub sets: BifixSetCache,
}

impl DataCache {
    pub fn precompute_up_to(&mut self, alphabet_size: u32, radius: u32) {
        // the max word size for a given radius is n + 1
        self.sets.precompute(alphabet_size, radius + 1);
        self.derangements
            .precompute_up_to(self.sets.max_set_size as u32);
    }
}
