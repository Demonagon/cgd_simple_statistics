use crate::math::word::Word;

fn self_bifix_free_words(alphabet_size: u32, word_size: u32) -> impl Iterator<Item = Word> {
    let count = alphabet_size.pow(word_size);

    (0..count)
        .map(move |mut k| {
            let mut result = Word::new([]);

            for _ in 0..word_size {
                result.data.push(k.rem_euclid(alphabet_size) as u8);
                k = k.div_euclid(alphabet_size);
            }

            result
        })
        .filter(|word| word.is_self_bifix_free())
}

fn can_include(word: &Word, set: &[Word]) -> bool {
    set.iter().all(|other| word.is_bifix_free_with(other))
}

/// Returns true if a is included in b or vice versa.
/// Makes the assumption that both sets are ordered
/// similarly.
fn are_sets_comparable(a: &[Word], b: &[Word]) -> bool {
    let mut a_k = 0;
    let mut b_k = 0;

    todo!()
}

pub fn bifix_free_sets(alphabet_size: u32, max_word_size: u32) -> Vec<Vec<Word>> {
    let words = (1..=max_word_size)
        .flat_map(|n| self_bifix_free_words(alphabet_size, n))
        .collect::<Vec<_>>();

    let mut result = vec![];

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
            }
        }

        // and we move on
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    use super::*;
    use crate::math::{permutation::Permutation, word::Word};

    #[test]
    fn test_self_bifix_free_words() {
        assert_eq!(
            self_bifix_free_words(2, 1).collect::<Vec<_>>(),
            vec![Word::new([0]), Word::new([1])]
        );

        assert_eq!(
            self_bifix_free_words(2, 2).collect::<Vec<_>>(),
            vec![Word::new([1, 0]), Word::new([0, 1])]
        );

        assert_eq!(
            self_bifix_free_words(2, 3).collect::<Vec<_>>(),
            vec![
                Word::new([1, 0, 0]),
                Word::new([1, 1, 0]),
                Word::new([0, 0, 1]),
                Word::new([0, 1, 1])
            ]
        );

        assert_eq!(
            self_bifix_free_words(2, 4).collect::<Vec<_>>(),
            vec![
                Word::new([1, 0, 0, 0]),
                Word::new([1, 1, 0, 0]),
                Word::new([1, 1, 1, 0]),
                Word::new([0, 0, 0, 1]),
                Word::new([0, 0, 1, 1]),
                Word::new([0, 1, 1, 1]),
            ]
        );

        assert_eq!(
            self_bifix_free_words(2, 5).collect::<Vec<_>>(),
            vec![
                Word::new([1, 0, 0, 0, 0]),
                Word::new([1, 1, 0, 0, 0]),
                Word::new([1, 0, 1, 0, 0]),
                Word::new([1, 1, 1, 0, 0]),
                Word::new([1, 1, 0, 1, 0]),
                Word::new([1, 1, 1, 1, 0]),
                Word::new([0, 0, 0, 0, 1]),
                Word::new([0, 0, 1, 0, 1]),
                Word::new([0, 0, 0, 1, 1]),
                Word::new([0, 1, 0, 1, 1]),
                Word::new([0, 0, 1, 1, 1]),
                Word::new([0, 1, 1, 1, 1]),
            ]
        );
    }

    #[test]
    fn test_enumeration() {
        assert_eq!(bifix_free_sets(2, 5).len(), 13);
        assert_eq!(bifix_free_sets(3, 2).len(), 16);

        assert!(bifix_free_sets(2, 5).is_sorted());
        assert!(bifix_free_sets(3, 2).is_sorted());
    }
}
