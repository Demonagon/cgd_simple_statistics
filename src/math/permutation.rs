use std::fmt::Display;

use crate::math::word::Word;

use smallvec::{SmallVec, smallvec};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Permutation {
    pub set: Vec<Word>,
    pub map: Vec<usize>,
}

impl Display for Permutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.set.is_empty() {
            f.write_str("()")?;
            return Ok(());
        }

        let mut index = 0;

        f.write_fmt(format_args!("{}", self.set[index]))?;

        loop {
            index = self.map[index];
            if index == 0 {
                break;
            }

            f.write_fmt(format_args!(" -> {}", self.set[index]))?;
        }

        Ok(())
    }
}

pub fn rules_radius(rules: &[Permutation]) -> u32 {
    let depth = rules.len();
    let Some(max_word_size) = rules
        .iter()
        .flat_map(|perm| perm.set.iter().map(|word| word.len()))
        .max()
    else {
        return 0;
    };

    if max_word_size == 0 {
        return 0;
    }

    depth as u32 * (max_word_size as u32 - 1)
}

pub fn apply_rules(rules: &[Permutation], mut word: Word) -> Word {
    for rule in rules {
        word = rule.apply(&word);
    }

    word
}

impl Permutation {
    /// Tries to advance into our reading of each word of the permutation
    /// by the provided letter. If words are interrupted, their progress
    /// go back to 0.
    fn progress_reading(&self, progress: &mut [usize], letter: u8) {
        progress
            .iter_mut()
            .enumerate()
            .for_each(|(word_index, progress)| {
                if self.set[word_index].data[*progress] == letter {
                    *progress += 1;
                } else if self.set[word_index].data[0] == letter {
                    // we failed, but we can directly start again
                    *progress = 1;
                } else {
                    *progress = 0;
                }
            })
    }

    fn found_word(&self, progress: &[usize]) -> Option<usize> {
        for (word_index, progress) in progress.iter().enumerate() {
            if self.set[word_index].data.len() == *progress {
                return Some(word_index);
            }
        }

        None
    }

    // returns true if some currently read word has progress that goes over
    // the 0 boundary. When false and we already did a complete loop,
    // the process can be soundely stopped.
    // Index is the index of the letter that was just added to the progress.
    fn progress_overlaps_boundary(progress: &[usize], index: usize) -> bool {
        progress.iter().any(|p| *p > index + 1)
    }

    /// Updates the configuration given in parameter using this
    /// permutation. The new configuration is returned.
    pub fn apply(&self, configuration: &Word) -> Word {
        let mut result = Word::default();

        let conf_len = configuration.data.len();

        // we track the progress of reading each word
        let mut progress: SmallVec<[usize; 8]> = smallvec![0; self.set.len()];

        // we work in two phases : first we need to detect a first word.
        // If we fail to do so, a copy of the input word is returned.
        // Once we find a word, we write its map into the result,
        // and we go to phase 2, where we continue to look for words
        // until we go back to the start of the first found word,
        // wrapping around the configuration if necessary.

        let mut index = 0;
        let first_word_index;
        let mut end_of_last_word;

        let mut loop_completed = false;

        println!("updating");
        println!("{configuration}");

        loop {
            println!("index {index}");
            self.progress_reading(&mut progress, configuration.data[index]);

            println!("progress {progress:?}");

            // we found our first word
            if let Some(word_index) = self.found_word(&progress) {
                println!("word found");
                // we reset the progress
                progress.iter_mut().for_each(|p| *p = 0);

                // we write the output to the result
                result
                    .data
                    .extend_from_slice(&self.set[self.map[word_index]].data);

                end_of_last_word = index;
                first_word_index =
                    (index + conf_len - self.set[word_index].data.len() + 1).rem_euclid(conf_len);
                index = (index + 1).rem_euclid(conf_len);
                break;
            }

            // we found nothing
            if loop_completed && !Permutation::progress_overlaps_boundary(&progress, index) {
                println!("found nothing");
                return configuration.clone();
            }

            index = (index + 1).rem_euclid(conf_len);

            if index == 0 {
                println!("completed first loop");
                loop_completed = true;
            }
        }

        while index != first_word_index {
            self.progress_reading(&mut progress, configuration.data[index]);

            // we found a new word
            if let Some(word_index) = self.found_word(&progress) {
                // we reset the progress
                progress.iter_mut().for_each(|p| *p = 0);

                let start_of_new_word =
                    (index + conf_len - self.set[word_index].data.len() + 1).rem_euclid(conf_len);

                // we write whatever was between the last word and the new word to the result
                let mut k = (end_of_last_word + 1).rem_euclid(conf_len);
                while k != start_of_new_word {
                    result.data.push(configuration.data[k]);

                    k = (k + 1).rem_euclid(conf_len);
                }

                // we write the output to the result
                result
                    .data
                    .extend_from_slice(&self.set[self.map[word_index]].data);

                end_of_last_word = index;
            }

            index = (index + 1).rem_euclid(conf_len);
        }

        // we made a loop, now we put back the leftovers in the result
        let mut k = (end_of_last_word + 1).rem_euclid(conf_len);
        while k != first_word_index {
            result.data.push(configuration.data[k]);

            k = (k + 1).rem_euclid(conf_len);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let perm = Permutation {
            set: vec![
                Word::new([2, 0, 1]),
                Word::new([2, 1, 0]),
                Word::new([2, 1, 1]),
                Word::new([2, 0, 0]),
            ],
            map: vec![1, 2, 3, 0],
        };

        assert_eq!(format!("{perm}"), "201 -> 210 -> 211 -> 200");
    }

    #[test]
    fn progress_reading() {
        let perm = Permutation {
            set: vec![
                Word::new([2, 0, 1]),
                Word::new([2, 1, 0]),
                Word::new([2, 1, 1]),
                Word::new([2, 0, 0]),
            ],
            map: vec![1, 2, 3, 0],
        };

        let mut progress = vec![0; 4];

        assert_eq!(perm.found_word(&progress), None);

        perm.progress_reading(&mut progress, 2);
        assert_eq!(progress, vec![1, 1, 1, 1]);
        assert_eq!(perm.found_word(&progress), None);

        perm.progress_reading(&mut progress, 1);
        assert_eq!(progress, vec![0, 2, 2, 0]);
        assert_eq!(perm.found_word(&progress), None);

        perm.progress_reading(&mut progress, 0);
        assert_eq!(progress, vec![0, 3, 0, 0]);
        assert_eq!(perm.found_word(&progress), Some(1));
    }

    #[test]
    fn apply() {
        let x = Word::new([0, 1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let perm = Permutation {
            set: vec![Word::new([0, 1]), Word::new([2])],
            map: vec![1, 0],
        };

        let y = perm.apply(&x);

        assert_eq!(y, Word::new([2, 1, 2, 1, 0, 0, 2]));

        let z = perm.apply(&y);

        assert_eq!(z, x);

        let x = Word::new([0, 0, 0, 0, 1, 1, 1, 1]);
        let perm = Permutation {
            set: vec![Word::new([1, 0]), Word::new([2])],
            map: vec![1, 0],
        };
        assert_eq!(perm.apply(&x), Word::new([2, 0, 0, 0, 1, 1, 1]));
        let x = Word::new([0, 0, 0, 1, 0, 1, 1, 1]);
        assert_eq!(perm.apply(&x), Word::new([2, 1, 1, 2, 0, 0]));
    }
}
