use indicatif::ParallelProgressIterator;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::math::{graph::Graph, tree::Tree, word::Word};

#[derive(Debug, PartialEq, Eq)]
pub struct DoubleTree {
    words: Vec<Word>,
    forward: Tree,
    backward: Tree,
}

/// Iterates the intersection of two ordered iterators
struct IntersectionIterator<I: Iterator<Item = usize>, J: Iterator<Item = usize>> {
    first: I,
    second: J,
}

impl<I: Iterator<Item = usize>, J: Iterator<Item = usize>> Iterator for IntersectionIterator<I, J> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.first.next()?;
        let mut j = self.second.next()?;

        while i != j {
            if i < j {
                i = self.first.next()?;
            } else {
                j = self.second.next()?;
            }
        }

        Some(i)
    }
}

impl DoubleTree {
    pub fn new(alphabet_size: usize, mut words: Vec<Word>) -> Self {
        let mut forward = Tree::new(alphabet_size);
        let mut backward = Tree::new(alphabet_size);

        words.par_sort_unstable();

        for k in 0..words.len() {
            let word = &words[k];
            forward.insert(k, word.clone());
            backward.insert(k, word.reversed());
        }

        Self {
            words,
            forward,
            backward,
        }
    }

    pub fn into_graph(self) -> Graph<Word> {
        let edges = self
            .words
            .par_iter()
            .progress()
            .map(|word| {
                let mut forward_indexes = self.forward.prefix_free_iter(&word).collect::<Vec<_>>();
                let mut backward_indexes = self
                    .backward
                    .prefix_free_iter(&word.reversed())
                    .collect::<Vec<_>>();

                forward_indexes.par_sort_unstable();
                backward_indexes.par_sort_unstable();

                IntersectionIterator {
                    first: forward_indexes.into_iter(),
                    second: backward_indexes.into_iter(),
                }
                .collect()
            })
            .collect();

        Graph {
            values: self.words,
            edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{
        set::{compute_words_graph, self_bifix_free_words},
        tree::Node,
    };
    use smallvec::smallvec;

    use super::*;

    #[test]
    fn intersection_iterator() {
        let a = [0, 1, 3, 4, 6];
        let b = [0, 1, 2, 3, 4];

        let mut iter = IntersectionIterator {
            first: a.into_iter(),
            second: b.into_iter(),
        };

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn double_tree_creation() {
        let words = vec![
            Word::new([0]),
            Word::new([0, 1]),
            Word::new([1]),
            Word::new([1, 0]),
        ];

        let double_tree = DoubleTree::new(2, words);

        let reference = DoubleTree {
            words: vec![
                Word::new([0]),
                Word::new([1]),
                Word::new([0, 1]),
                Word::new([1, 0]),
            ],
            forward: Tree {
                alphabet_size: 2,
                nodes: vec![
                    Node {
                        word_index: None,
                        prefix: vec![],
                        children: smallvec![Some(1), Some(2)],
                    },
                    Node {
                        word_index: Some(0),
                        prefix: vec![0],
                        children: smallvec![None, Some(3)],
                    },
                    Node {
                        word_index: Some(1),
                        prefix: vec![1],
                        children: smallvec![Some(4), None],
                    },
                    Node {
                        word_index: Some(2),
                        prefix: vec![1],
                        children: smallvec![None, None],
                    },
                    Node {
                        word_index: Some(3),
                        prefix: vec![0],
                        children: smallvec![None, None],
                    },
                ],
            },
            backward: Tree {
                alphabet_size: 2,
                nodes: vec![
                    Node {
                        word_index: None,
                        prefix: vec![],
                        children: smallvec![Some(1), Some(2)],
                    },
                    Node {
                        word_index: Some(0),
                        prefix: vec![0],
                        children: smallvec![None, Some(4)],
                    },
                    Node {
                        word_index: Some(1),
                        prefix: vec![1],
                        children: smallvec![Some(3), None],
                    },
                    Node {
                        word_index: Some(2),
                        prefix: vec![0],
                        children: smallvec![None, None],
                    },
                    Node {
                        word_index: Some(3),
                        prefix: vec![1],
                        children: smallvec![None, None],
                    },
                ],
            },
        };

        assert_eq!(double_tree.words, reference.words);
        assert_eq!(double_tree.forward, reference.forward);
        assert_eq!(double_tree.backward, reference.backward);
    }

    #[test]
    fn test_into_graph() {
        let words = vec![
            Word::new([0]),
            Word::new([0, 1]),
            Word::new([1]),
            Word::new([1, 0]),
        ];

        let graph = DoubleTree::new(2, words).into_graph();

        assert_eq!(
            graph,
            Graph {
                values: vec![
                    Word::new([0]),
                    Word::new([1]),
                    Word::new([0, 1]),
                    Word::new([1, 0]),
                ],
                edges: vec![smallvec![1], smallvec![0], smallvec![], smallvec![],],
            }
        );
    }

    #[test]
    fn large_scale_test() {
        let alphabet_size = 3;
        let radius = 3;

        let mut words = vec![];

        for r in 0..=radius {
            words.extend(self_bifix_free_words(alphabet_size, r + 1));
        }

        let reference = compute_words_graph(&words);
        let value = DoubleTree::new(alphabet_size as usize, words).into_graph();

        assert_eq!(reference.values, value.values);
        assert_eq!(reference.edges, value.edges);
    }
}
