use rand::{Rng, seq::IndexedRandom};

use crate::math::{permutation::Permutation, word::Word};

fn decompositions(a: u32) -> Vec<(u32, u32)> {
    let mut result = vec![];

    for d in 1..=a {
        if a.is_multiple_of(d) {
            result.push((d, a / d));
        }
    }

    result
}

fn generate_shift(left: u32, right: u32) -> Vec<Permutation> {
    let left = left as u8;
    let right = right as u8;
    let alphabet_size = left * right;
    let offset = alphabet_size;
    let separator = alphabet_size + offset;

    let mut decomposition_set = vec![];
    let mut decomposition_map = vec![];
    for k in 0..alphabet_size {
        decomposition_set.push(Word::new([k]));
        decomposition_set.push(Word::new([separator, offset + k % left, offset + k / left]));

        let last = decomposition_map.len();
        decomposition_map.push(last + 1);
        decomposition_map.push(last);
    }

    let decomposition = Permutation {
        set: decomposition_set,
        map: decomposition_map,
    };

    let recomposition = decomposition.clone();

    // swap & pivot
    let mut swap_set = vec![];
    let mut pivot_set = vec![];
    let mut map = vec![];

    let max = left.max(right);

    for l in 0..max {
        for r in (l + 1)..max {
            swap_set.push(Word::new([offset + l, offset + r]));
            swap_set.push(Word::new([offset + r, offset + l]));

            pivot_set.push(Word::new([offset + l, separator, offset + r]));
            pivot_set.push(Word::new([offset + r, separator, offset + l]));

            let last = map.len();
            map.push(last + 1);
            map.push(last);
        }
    }

    let swap = Permutation {
        set: swap_set,
        map: map.clone(),
    };

    let pivot = Permutation {
        set: pivot_set,
        map,
    };

    vec![decomposition, swap, pivot, recomposition]
}

pub fn generate_random_shift(alphabet_size: u32, rng: &mut impl Rng) -> Vec<Permutation> {
    let (left, right) = decompositions(alphabet_size)
        .choose(rng)
        .copied()
        .expect("Failed to select random decomposition for a shift");

    generate_shift(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift() {
        assert_eq!(
            generate_shift(1, 1),
            vec![
                Permutation {
                    set: vec![Word::new([0]), Word::new([2, 1, 1])],
                    map: vec![1, 0],
                },
                Permutation {
                    set: vec![],
                    map: vec![]
                },
                Permutation {
                    set: vec![],
                    map: vec![]
                },
                Permutation {
                    set: vec![Word::new([0]), Word::new([2, 1, 1])],
                    map: vec![1, 0],
                },
            ]
        );

        assert_eq!(
            generate_shift(2, 3),
            vec![
                Permutation {
                    set: vec![
                        Word::new([0]),
                        Word::new([12, 6, 6]),
                        Word::new([1]),
                        Word::new([12, 7, 6]),
                        Word::new([2]),
                        Word::new([12, 6, 7]),
                        Word::new([3]),
                        Word::new([12, 7, 7]),
                        Word::new([4]),
                        Word::new([12, 6, 8]),
                        Word::new([5]),
                        Word::new([12, 7, 8]),
                    ],
                    map: vec![1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10],
                },
                Permutation {
                    set: vec![
                        Word::new([6, 7]),
                        Word::new([7, 6]),
                        Word::new([6, 8]),
                        Word::new([8, 6]),
                        Word::new([7, 8]),
                        Word::new([8, 7]),
                    ],
                    map: vec![1, 0, 3, 2, 5, 4],
                },
                Permutation {
                    set: vec![
                        Word::new([6, 12, 7]),
                        Word::new([7, 12, 6]),
                        Word::new([6, 12, 8]),
                        Word::new([8, 12, 6]),
                        Word::new([7, 12, 8]),
                        Word::new([8, 12, 7]),
                    ],
                    map: vec![1, 0, 3, 2, 5, 4],
                },
                Permutation {
                    set: vec![
                        Word::new([0]),
                        Word::new([12, 6, 6]),
                        Word::new([1]),
                        Word::new([12, 7, 6]),
                        Word::new([2]),
                        Word::new([12, 6, 7]),
                        Word::new([3]),
                        Word::new([12, 7, 7]),
                        Word::new([4]),
                        Word::new([12, 6, 8]),
                        Word::new([5]),
                        Word::new([12, 7, 8]),
                    ],
                    map: vec![1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10],
                },
            ]
        );
    }
}
