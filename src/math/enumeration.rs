use std::collections::HashMap;

use crate::math::cache::DataCache;

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
/// This a pretty expensive operation and should be only done once.
pub fn count_rules(cache: &DataCache, alphabet_size: u32, radius: u32) -> u32 {
    divisors(radius)
        .map(|depth| count_possibilities(cache, alphabet_size, depth, radius.div_euclid(depth) + 1))
        .sum()
}

#[cfg(test)]
mod tests {
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
}
