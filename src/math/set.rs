use rand::{rngs::ThreadRng, seq::IndexedRandom};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::math::{graph::Graph, word::Word};

fn self_bifix_free_words(alphabet_size: u32, word_size: u32) -> impl Iterator<Item = Word> {
    let count = alphabet_size.pow(word_size);

    (0..count)
        .map(move |mut k| {
            let mut result = Word::new([]);

            for _ in 0..word_size {
                result.data.push(k.rem_euclid(alphabet_size) as u8);
                k = k.div_euclid(alphabet_size);
            }

            Word::new(result.data.into_iter().rev())
        })
        .filter(|word| word.is_self_bifix_free())
}

fn can_include(word: &Word, set: &[Word]) -> bool {
    set.iter().all(|other| word.is_bifix_free_with(other))
}

/// returns true if a includes b
fn includes(a: &[Word], b: &[Word]) -> bool {
    let mut k = 0;

    for w in b {
        loop {
            if k >= a.len() {
                return false;
            }

            if &a[k] == w {
                break;
            }

            k += 1;
        }
    }

    true
}

/// Compares set in inclusion terms.
/// Makes the assumption that both sets are ordered.
fn compare_sets(a: &[Word], b: &[Word]) -> Option<std::cmp::Ordering> {
    match (includes(a, b), includes(b, a)) {
        (true, true) => Some(std::cmp::Ordering::Equal),
        (true, false) => Some(std::cmp::Ordering::Greater),
        (false, true) => Some(std::cmp::Ordering::Less),
        (false, false) => None,
    }
}

pub fn is_sequence_valid(sequence: &[Vec<Word>]) -> bool {
    if sequence.is_empty() {
        return true;
    }

    for w in sequence.windows(2) {
        if compare_sets(&w[0], &w[1]).is_some() {
            return false;
        }
    }

    compare_sets(sequence.first().unwrap(), sequence.last().unwrap()).is_none()
}

fn compute_bifix_free_sets(words: &[Word]) -> Vec<Vec<Word>> {
    // includes the empty set
    let mut result = vec![vec![]];

    // both vecs are kept in sync; the first contains the words, the second contains their index in words
    let mut current_set = vec![];
    let mut current_indexes = vec![];

    let mut index = 0;

    loop {
        // we can't add more; we need to find another set
        if index >= words.len() {
            // we try to remove the last word
            let Some(last_index) = current_indexes.pop() else {
                // the process is over
                break;
            };

            _ = current_set.pop();
            index = last_index + 1;

            continue;
        }

        // we try to add the current word
        if can_include(&words[index], &current_set) {
            current_set.push(words[index].clone());
            current_indexes.push(index);

            // we completed a set
            if current_set.len() > 1 {
                result.push(current_set.clone());
                // DEBUG : for seeing progress
                // if result.len() % 100 == 0 {
                //     print!(".");
                //     let _ = std::io::stdout().flush();
                // }
            }
        }

        // and we move on
        index += 1;
    }

    result
}

pub fn sample_bifix_free_set(
    word_graph: &Graph<Word>,
    rng: &mut ThreadRng,
    initial_words: &[usize],
) -> Vec<Word> {
    let mut clique: Vec<usize> = vec![];

    let Some(&initial_index) = initial_words.choose(rng) else {
        return vec![];
    };

    clique.push(initial_index);

    loop {
        let neighbors = word_graph.shared_neigbhors(&clique);

        let Some(&new_index) = neighbors.choose(rng) else {
            break;
        };

        clique.push(new_index);
    }

    clique
        .iter()
        .map(|&k| word_graph.get(k).expect("Error in set sampling").clone())
        .collect()
}

pub fn bifix_free_sets(alphabet_size: u32, max_word_size: u32) -> Vec<Vec<Word>> {
    let words = (1..=max_word_size)
        .flat_map(|n| self_bifix_free_words(alphabet_size, n))
        .collect::<Vec<_>>();

    compute_bifix_free_sets(&words)
}

pub fn number_conserving_bifix_free_sets(alphabet_size: u32, max_word_size: u32) -> Vec<Vec<Word>> {
    let words = (1..=max_word_size)
        .flat_map(|n| self_bifix_free_words(alphabet_size, n))
        .collect::<Vec<_>>();

    let max_value_sum = words.iter().map(|w| w.value_sum()).max().unwrap();

    let mut sum_sorted_words: Vec<Vec<Word>> = vec![vec![]; max_value_sum as usize + 1];

    for word in words {
        sum_sorted_words[word.value_sum() as usize].push(word);
    }

    sum_sorted_words
        .into_par_iter()
        .map(|words: Vec<Word>| compute_bifix_free_sets(&words))
        .flatten()
        .filter(|set| !set.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::math::word::Word;

    #[test]
    fn test_self_bifix_free_words() {
        assert_eq!(
            self_bifix_free_words(2, 1).collect::<Vec<_>>(),
            vec![Word::new([0]), Word::new([1])]
        );

        assert_eq!(
            self_bifix_free_words(2, 2).collect::<Vec<_>>(),
            vec![Word::new([0, 1]), Word::new([1, 0])]
        );

        assert_eq!(
            self_bifix_free_words(2, 3).collect::<Vec<_>>(),
            vec![
                Word::new([0, 0, 1]),
                Word::new([0, 1, 1]),
                Word::new([1, 0, 0]),
                Word::new([1, 1, 0]),
            ]
        );

        assert_eq!(
            self_bifix_free_words(2, 4).collect::<Vec<_>>(),
            vec![
                Word::new([0, 0, 0, 1]),
                Word::new([0, 0, 1, 1]),
                Word::new([0, 1, 1, 1]),
                Word::new([1, 0, 0, 0]),
                Word::new([1, 1, 0, 0]),
                Word::new([1, 1, 1, 0]),
            ]
        );

        assert_eq!(
            self_bifix_free_words(2, 5).collect::<Vec<_>>(),
            vec![
                Word::new([0, 0, 0, 0, 1]),
                Word::new([0, 0, 0, 1, 1]),
                Word::new([0, 0, 1, 0, 1]),
                Word::new([0, 0, 1, 1, 1]),
                Word::new([0, 1, 0, 1, 1]),
                Word::new([0, 1, 1, 1, 1]),
                Word::new([1, 0, 0, 0, 0]),
                Word::new([1, 0, 1, 0, 0]),
                Word::new([1, 1, 0, 0, 0]),
                Word::new([1, 1, 0, 1, 0]),
                Word::new([1, 1, 1, 0, 0]),
                Word::new([1, 1, 1, 1, 0]),
            ]
        );
    }

    #[test]
    fn test_order() {
        let max_word_size = 5;

        let words = (1..=max_word_size)
            .flat_map(|n| self_bifix_free_words(2, n))
            .collect::<Vec<_>>();

        assert!(words.is_sorted());
    }

    #[test]
    fn test_enumeration() {
        assert_eq!(bifix_free_sets(2, 5).len(), 14);
        assert_eq!(bifix_free_sets(3, 2).len(), 17);
    }

    #[test]
    fn test_compare_sets() {
        assert_eq!(
            compare_sets(&[Word::new([0]), Word::new([1])], &[Word::new([1])]),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            compare_sets(&[Word::new([0]), Word::new([2])], &[Word::new([1])]),
            None
        );
        assert_eq!(
            compare_sets(
                &[Word::new([0]), Word::new([1])],
                &[Word::new([0]), Word::new([1])]
            ),
            Some(std::cmp::Ordering::Equal)
        );
        assert_eq!(
            compare_sets(
                &[Word::new([0]), Word::new([2])],
                &[Word::new([0]), Word::new([1])]
            ),
            None
        );
    }

    #[test]
    fn number_conserving_sets() {
        let sets = number_conserving_bifix_free_sets(3, 3);

        assert_eq!(sets.len(), 16);

        assert_eq!(sets[0], vec![Word::new([2]), Word::new([0, 1, 1])]);
        assert_eq!(sets[1], vec![Word::new([2]), Word::new([1, 1, 0])]);
        assert_eq!(sets[2], vec![Word::new([0, 2]), Word::new([0, 1, 1])]);
        assert_eq!(sets[3], vec![Word::new([2, 0]), Word::new([1, 1, 0])]);
        assert_eq!(sets[4], vec![Word::new([0, 0, 2]), Word::new([0, 1, 1])]);
        assert_eq!(sets[5], vec![Word::new([1, 1, 0]), Word::new([2, 0, 0])]);
        assert_eq!(sets[6], vec![Word::new([1, 2]), Word::new([1, 0, 2])]);
        assert_eq!(sets[7], vec![Word::new([2, 1]), Word::new([2, 0, 1])]);
        assert_eq!(sets[8], vec![Word::new([0, 1, 2]), Word::new([0, 2, 1])]);
        assert_eq!(sets[9], vec![Word::new([0, 1, 2]), Word::new([1, 0, 2])]);
        assert_eq!(sets[10], vec![Word::new([0, 2, 1]), Word::new([2, 0, 1])]);
        assert_eq!(sets[11], vec![Word::new([1, 0, 2]), Word::new([1, 2, 0])]);
        assert_eq!(sets[12], vec![Word::new([1, 2, 0]), Word::new([2, 1, 0])]);
        assert_eq!(sets[13], vec![Word::new([2, 0, 1]), Word::new([2, 1, 0])]);
        assert_eq!(sets[14], vec![Word::new([0, 2, 2]), Word::new([1, 1, 2])]);
        assert_eq!(sets[15], vec![Word::new([2, 1, 1]), Word::new([2, 2, 0])]);
    }
}
