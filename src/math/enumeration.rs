use std::collections::{HashMap, btree_map::Range};

use crate::math::{cache::DataCache, permutation::Permutation};

fn divisors(n: u32) -> impl Iterator<Item = u32> {
    (1..=n).filter(move |&k| n.rem_euclid(k) == 0)
}

fn count_possibilities(
    cache: &DataCache,
    alphabet_size: u32,
    depth: u32,
    max_word_size: u32,
) -> u32 {
    let sets = cache
        .sets
        .get(alphabet_size, max_word_size)
        .expect("Please precompute bifix-free sets before counting rules");

    let mut lengths: HashMap<u32, u32> = HashMap::new();

    for set in sets {
        *lengths.entry(set.len() as u32).or_insert(0) += 1;
    }

    // earch length, we compute the number of ways we can create a single permutation:
    // that's number of sets of a given size times the number of derangements of that size,
    // summed over all sizes
    let depth_1_count: u32 = lengths
        .into_iter()
        .map(|(len, count)| {
            count
                * cache
                    .derangements
                    .get(len)
                    .expect("Please precompute derangements before counting rules")
                    .len() as u32
        })
        .sum();

    //The final number is then that to the power of the depth
    depth_1_count.pow(depth)
}

/// Counts how many rules can be generated given these parameters.
/// This a pretty expensive operation and should be only done rarely.
pub fn count_rules(cache: &DataCache, alphabet_size: u32, radius: u32) -> u32 {
    divisors(radius)
        .map(|depth| count_possibilities(cache, alphabet_size, depth, radius.div_euclid(depth) + 1))
        .sum()
}

pub fn all_rule_indexes(
    cache: &DataCache,
    alphabet_size: u32,
    radius: u32,
) -> impl Iterator<Item = u32> {
    0..count_rules(cache, alphabet_size, radius)
}

pub fn rule_from_index(
    cache: &DataCache,
    alphabet_size: u32,
    radius: u32,
    mut number: u32,
) -> Vec<Permutation> {
    let divisors = divisors(radius).collect::<Vec<_>>();

    let depth = divisors[(number % divisors.len() as u32) as usize];
    number /= divisors.len() as u32;

    let max_word_size = radius / depth + 1;

    let mut result = vec![];

    // 1 17 2

    for _ in 0..depth {
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
    fn test_divisors() {
        assert_eq!(divisors(0).collect::<Vec<_>>(), vec![]);
        assert_eq!(divisors(1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(divisors(2).collect::<Vec<_>>(), vec![1, 2]);

        assert_eq!(divisors(5).collect::<Vec<_>>(), vec![1, 5]);
        assert_eq!(divisors(6).collect::<Vec<_>>(), vec![1, 2, 3, 6]);
        assert_eq!(divisors(12).collect::<Vec<_>>(), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_rule_counting() {
        let mut cache = DataCache::default();
        cache.precompute_up_to(3, 1);

        // all radius 0:
        // - identity,
        // - 0 -> 1
        // - 0 -> 1 -> 2 (x2 derangements)
        // - 0 -> 2
        // - 1 -> 2
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

        assert_eq!(count_rules(&cache, 3, 1), 18);
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

        assert_eq!(
            rule_from_index(&cache, 3, 1, 0),
            vec![Permutation {
                set: vec![],
                map: vec![]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 1),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 2),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![2, 0, 1]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 3),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1]), Word::new([2])],
                map: vec![1, 2, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 4),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 5),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([1, 2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 6),
            vec![Permutation {
                set: vec![Word::new([0]), Word::new([2, 1])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 7),
            vec![Permutation {
                set: vec![Word::new([1]), Word::new([2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 8),
            vec![Permutation {
                set: vec![Word::new([1]), Word::new([0, 2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 9),
            vec![Permutation {
                set: vec![Word::new([1]), Word::new([2, 0])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 10),
            vec![Permutation {
                set: vec![Word::new([2]), Word::new([0, 1])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 11),
            vec![Permutation {
                set: vec![Word::new([2]), Word::new([1, 0])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 12),
            vec![Permutation {
                set: vec![Word::new([0, 1]), Word::new([0, 2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 13),
            vec![Permutation {
                set: vec![Word::new([0, 1]), Word::new([2, 1])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 14),
            vec![Permutation {
                set: vec![Word::new([0, 2]), Word::new([1, 2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 15),
            vec![Permutation {
                set: vec![Word::new([1, 0]), Word::new([1, 2])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 16),
            vec![Permutation {
                set: vec![Word::new([1, 0]), Word::new([2, 0])],
                map: vec![1, 0]
            }]
        );

        assert_eq!(
            rule_from_index(&cache, 3, 1, 17),
            vec![Permutation {
                set: vec![Word::new([2, 0]), Word::new([2, 1])],
                map: vec![1, 0]
            }]
        );
    }
}
