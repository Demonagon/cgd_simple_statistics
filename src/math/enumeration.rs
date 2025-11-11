use std::ops::Range;

use crate::math::{cache::DataCache, derangement::factorial, permutation::Permutation};

fn count_number_conserving_possibilities(
    cache: &DataCache,
    alphabet_size: u32,
    set_sizes: &[u32],
) -> u32 {
    // there is no unary rules in number conserving world
    let mut total = 1;

    for size in set_sizes {
        total *= cache
            .possibilities
            .permutation_possibilities
            .get(&(alphabet_size, *size + 1))
            .unwrap_or(&0)
    }

    total
}

/// Counts how many ways we can "populate" a given list of set sizes
fn count_possibilities(cache: &DataCache, alphabet_size: u32, set_sizes: &[u32]) -> u32 {
    // A unary rule is placed in front of every rule
    let mut total = factorial(alphabet_size);

    for size in set_sizes {
        total *= cache
            .possibilities
            .permutation_possibilities
            .get(&(alphabet_size, *size + 1))
            .unwrap_or(&0)
    }

    total
}

/// Counts how many number conserving rules can be generated given these parameters.
/// This a pretty expensive operation and should be only done rarely.
pub fn count_number_conserving_rules(cache: &DataCache, alphabet_size: u32, radius: u32) -> u32 {
    cache
        .sums
        .sums
        .get(&radius)
        .expect("Precompute sums before counting rules")
        .iter()
        .map(|sequence| count_number_conserving_possibilities(cache, alphabet_size, sequence))
        .sum::<u32>()
}

/// Counts how many rules can be generated given these parameters.
/// This a pretty expensive operation and should be only done rarely.
pub fn count_rules(cache: &DataCache, alphabet_size: u32, radius: u32) -> u32 {
    cache
        .sums
        .sums
        .get(&radius)
        .expect("Precompute sums before counting rules")
        .iter()
        .map(|sequence| count_possibilities(cache, alphabet_size, sequence))
        .sum::<u32>()
}

pub fn all_rule_indexes(cache: &DataCache, alphabet_size: u32, radius: u32) -> Range<u32> {
    0..count_rules(cache, alphabet_size, radius)
}

pub fn all_number_conserving_rule_indexes(
    cache: &DataCache,
    alphabet_size: u32,
    radius: u32,
) -> Range<u32> {
    0..count_number_conserving_rules(cache, alphabet_size, radius)
}

pub fn rule_from_index(
    cache: &DataCache,
    alphabet_size: u32,
    radius: u32,
    mut number: u32,
) -> Vec<Permutation> {
    let sequences = cache
        .sums
        .sums
        .get(&radius)
        .expect("Precompute sums before getting rule from index");
    let mut sequence_number = 0;

    // we find the current sequence
    while number >= count_possibilities(cache, alphabet_size, &sequences[sequence_number]) {
        number -= count_possibilities(cache, alphabet_size, &sequences[sequence_number]);
        sequence_number += 1;
    }

    let sequence = &sequences[sequence_number];

    let mut result = vec![];

    for value in sequence {
        let max_word_size = value + 1;

        let sets = cache
            .sets
            .get(alphabet_size, max_word_size)
            .expect("Please precompute sets before generating rules from indexes");

        // for each set we tabulate the number of derangements that match
        let possibilities = sets
            .iter()
            .map(|s| cache.derangements.get(s.len() as u32).unwrap().len() as u32)
            .collect::<Vec<_>>();

        let total_possibilities: u32 = possibilities.iter().sum();
        let mut sub_number = number % total_possibilities;
        number /= total_possibilities;

        // we go through the table with our number
        let mut index = 0;
        while sub_number >= possibilities[index] {
            sub_number -= possibilities[index];
            index += 1;
        }

        let set = sets[index].clone();
        let map = cache
            .derangements
            .get(set.len() as u32)
            .expect("Please precompute derangements before generating rules from indexes")
            [sub_number as usize]
            .clone();

        result.push(Permutation { set, map });
    }

    // we add the "prefix" unary operation,
    // that happens to be put at the end

    let prefix_number = number % factorial(alphabet_size);

    result.push(Permutation::unary(alphabet_size, prefix_number));

    result
}

pub fn number_conserving_rule_from_index(
    cache: &DataCache,
    alphabet_size: u32,
    radius: u32,
    mut number: u32,
) -> Vec<Permutation> {
    let sequences = cache
        .sums
        .sums
        .get(&radius)
        .expect("Precompute sums before getting rule from index");
    let mut sequence_number = 0;

    // we find the current sequence
    while number
        >= count_number_conserving_possibilities(cache, alphabet_size, &sequences[sequence_number])
    {
        number -= count_number_conserving_possibilities(
            cache,
            alphabet_size,
            &sequences[sequence_number],
        );
        sequence_number += 1;
    }

    let sequence = &sequences[sequence_number];

    let mut result = vec![];

    for value in sequence {
        let max_word_size = value + 1;

        let sets = cache
            .sets
            .get(alphabet_size, max_word_size)
            .expect("Please precompute sets before generating rules from indexes");

        // for each set we tabulate the number of derangements that match
        let possibilities = sets
            .iter()
            .map(|s| cache.derangements.get(s.len() as u32).unwrap().len() as u32)
            .collect::<Vec<_>>();

        let total_possibilities: u32 = possibilities.iter().sum();
        let mut sub_number = number % total_possibilities;
        number /= total_possibilities;

        // we go through the table with our number
        let mut index = 0;
        while sub_number >= possibilities[index] {
            sub_number -= possibilities[index];
            index += 1;
        }

        let set = sets[index].clone();
        let map = cache
            .derangements
            .get(set.len() as u32)
            .expect("Please precompute derangements before generating rules from indexes")
            [sub_number as usize]
            .clone();

        result.push(Permutation { set, map });
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::math::word::Word;

    use super::*;

    #[test]
    fn test_rule_counting() {
        let mut cache = DataCache::default();
        cache.precompute_up_to(3, 1);

        // all radius 0:
        // - identity,
        // - 0 <-> 1
        // - 0 <-> 1 <-> 2 (x2 derangements)
        // - 0 <-> 2
        // - 1 <-> 2

        assert_eq!(count_rules(&cache, 3, 0), 6);

        // all radius 1:
        // - 0 <-> 21
        // - 0 <-> 12
        // - 1 <-> 20
        // - 1 <-> 02
        // - 2 <-> 10
        // - 2 <-> 01
        // - 10 <-> 20
        // - 10 <-> 12
        // - 20 <-> 21
        // - 01 <-> 21
        // - 01 <-> 02
        // - 02 <-> 12

        // with unary prefix, we have 6 such prefixes

        assert_eq!(count_rules(&cache, 3, 1), 12 * 6);
    }

    #[test]
    fn test_number_conserving_rule_counting() {
        let mut cache = DataCache::default();
        cache.precompute_number_conserving_up_to(3, 2);

        // only the identity

        assert_eq!(count_number_conserving_rules(&cache, 3, 0), 1);

        // no radius 1:

        assert_eq!(count_number_conserving_rules(&cache, 3, 1), 0);

        // radius 2:

        assert_eq!(count_number_conserving_rules(&cache, 3, 2), 16);
    }

    #[test]
    fn test_rule_deriving() {
        let mut cache = DataCache::default();
        cache.precompute_up_to(3, 1);

        // all radius 0:
        // - identity,
        // - 0 -> 1
        // - 0 -> 1 -> 2 (x2 derangements)
        // - 0 -> 2
        // - 1 -> 2

        assert_eq!(
            rule_from_index(&cache, 3, 0, 0),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![0, 1, 2],
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 0, 1),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![1, 0, 2],
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 0, 2),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![2, 0, 1],
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 0, 3),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![0, 2, 1],
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 0, 4),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![1, 2, 0],
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 0, 5),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![2, 1, 0],
            }]
        );

        // all radius 1:
        // - 0 -> 21
        // - 0 -> 12
        // - 1 -> 20
        // - 1 -> 02
        // - 2 -> 10
        // - 2 -> 01
        // - 10 -> 20
        // - 10 -> 12
        // - 20 -> 21
        // - 01 -> 21
        // - 01 -> 02
        // - 02 -> 12
        //
        // x 6 for each unary operator

        assert_eq!(
            rule_from_index(&cache, 3, 1, 0),
            vec![
                Permutation {
                    set: vec![Word::new([0]), Word::new([1, 2])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 1),
            vec![
                Permutation {
                    set: vec![Word::new([0]), Word::new([2, 1])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );
        assert_eq!(
            rule_from_index(&cache, 3, 1, 2),
            vec![
                Permutation {
                    set: vec![Word::new([1]), Word::new([0, 2])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 3),
            vec![
                Permutation {
                    set: vec![Word::new([1]), Word::new([2, 0])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 4),
            vec![
                Permutation {
                    set: vec![Word::new([2]), Word::new([0, 1])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 5),
            vec![
                Permutation {
                    set: vec![Word::new([2]), Word::new([1, 0])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 6),
            vec![
                Permutation {
                    set: vec![Word::new([0, 1]), Word::new([0, 2])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 7),
            vec![
                Permutation {
                    set: vec![Word::new([0, 1]), Word::new([2, 1])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 8),
            vec![
                Permutation {
                    set: vec![Word::new([0, 2]), Word::new([1, 2])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 9),
            vec![
                Permutation {
                    set: vec![Word::new([1, 0]), Word::new([1, 2])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 10),
            vec![
                Permutation {
                    set: vec![Word::new([1, 0]), Word::new([2, 0])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 11),
            vec![
                Permutation {
                    set: vec![Word::new([2, 0]), Word::new([2, 1])],
                    map: vec![1, 0]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                    map: vec![0, 1, 2],
                }
            ]
        );
    }
}
