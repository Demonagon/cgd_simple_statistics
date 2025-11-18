use std::{collections::HashMap, ops::Index};

use rand::{Rng, seq::IndexedRandom};
use smallvec::{SmallVec, smallvec};

use crate::math::word::Word;

const EDGES_CAPACITY: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph<T> {
    values: Vec<T>,
    edges: Vec<SmallVec<[usize; EDGES_CAPACITY]>>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            edges: Default::default(),
        }
    }
}

impl<T> Graph<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    pub fn push(&mut self, value: T) -> usize {
        self.values.push(value);
        self.edges.push(Default::default());
        self.len() - 1
    }

    pub fn push_edge(&mut self, a: usize, b: usize) -> Option<()> {
        self.push_directionnal_edge(a, b)?;
        self.push_directionnal_edge(b, a)
    }

    pub fn push_directionnal_edge(&mut self, from: usize, to: usize) -> Option<()> {
        if to >= self.len() {
            return None;
        }

        let edges = &mut self.edges.get_mut(from)?;

        edges.push(to);
        edges.sort_unstable();
        edges.dedup();

        Some(())
    }

    pub fn shared_neigbhors(&self, nodes: &[usize]) -> SmallVec<[usize; EDGES_CAPACITY]> {
        let mut result = smallvec![];

        let edges = nodes.iter().map(|k| &self.edges[*k]).collect::<Vec<_>>();
        let mut indices = vec![0; nodes.len()];

        let Some(max) = edges[0].first() else {
            return result;
        };

        let mut max = *max;

        'main_loop: loop {
            // we check no index goes beyond bounds
            for k in 0..indices.len() {
                if indices[k] >= edges[k].len() {
                    return result;
                }
            }

            // we check if every index is equal, failing that we update the max value
            for k in 0..nodes.len() {
                // if the value is less than the maximum, we update that index
                while edges[k][indices[k]] < max {
                    indices[k] += 1;
                    if indices[k] >= edges[k].len() {
                        return result;
                    }
                }

                if edges[k][indices[k]] > max {
                    // we found a new max value
                    max = edges[k][indices[k]];

                    // we go through the list again
                    continue 'main_loop;
                }
            }

            // we only get here if all values are the same
            result.push(max);

            indices.iter_mut().for_each(|k| *k += 1);
        }
    }
}

impl<T: Ord + Clone> Graph<T> {
    pub fn new_ordered_from_slice(slice: &[T]) -> Self {
        let mut values = Vec::from(slice);

        values.sort_unstable();

        Graph {
            values,
            edges: vec![Default::default(); slice.len()],
        }
    }
}

impl<T> Index<usize> for Graph<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl Graph<Word> {
    /// Assumes that the words contained in the graph are in order.
    /// Computes and returns a vec of indexes corresponding to each
    /// size of words. Excludes nodes which do not have neibhbors;
    /// we use these vecs to generate cliques of size at least 2.
    pub fn word_size_ranges(&self) -> HashMap<u32, Vec<usize>> {
        let mut result = HashMap::<u32, Vec<usize>>::default();

        for k in 0..self.len() {
            if self.edges[k].is_empty() {
                continue;
            }

            result.entry(self[k].len() as u32).or_default().push(k);
        }

        result
    }
}

impl<T> Graph<T> {
    pub fn sample_clique(&self, starting_index: usize, rng: &mut impl Rng) -> Vec<usize> {
        let mut result = vec![starting_index];

        loop {
            let candidates = self.shared_neigbhors(&result);
            match candidates.choose(rng) {
                Some(&index) => result.push(index),
                None => break,
            }
        }

        result.sort_unstable();

        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn graph_creation() {
        let mut graph = Graph::default();

        graph.push("a");
        graph.push("b");
        graph.push("c");
        graph.push("d");
        graph.push("e");

        graph.push_directionnal_edge(0, 1);
        graph.push_directionnal_edge(1, 2);
        graph.push_directionnal_edge(2, 0);

        graph.push_edge(3, 4);

        assert_eq!(
            graph,
            Graph {
                values: vec!["a", "b", "c", "d", "e"],
                edges: vec![
                    smallvec![1],
                    smallvec![2],
                    smallvec![0],
                    smallvec![4],
                    smallvec![3]
                ]
            }
        )
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Graph::default();

        graph.push("a");
        graph.push("b");
        graph.push("c");
        graph.push("d");
        graph.push("e");

        graph.push_edge(1, 0);
        graph.push_edge(2, 0);
        graph.push_edge(2, 1);
        graph.push_edge(3, 0);
        graph.push_edge(3, 1);
        graph.push_edge(3, 2);
        graph.push_edge(4, 0);
        graph.push_edge(4, 1);
        graph.push_edge(4, 2);
        graph.push_edge(4, 3);

        let neighbors: SmallVec<[usize; EDGES_CAPACITY]> = smallvec![4];
        assert_eq!(graph.shared_neigbhors(&[0, 1, 2, 3]), neighbors);

        let neighbors: SmallVec<[usize; EDGES_CAPACITY]> = smallvec![1, 4];
        assert_eq!(graph.shared_neigbhors(&[0, 2, 3]), neighbors);

        let neighbors: SmallVec<[usize; EDGES_CAPACITY]> = smallvec![1, 2, 3, 4];
        assert_eq!(graph.shared_neigbhors(&[0]), neighbors);

        let neighbors: SmallVec<[usize; EDGES_CAPACITY]> = smallvec![0, 1, 2];
        assert_eq!(graph.shared_neigbhors(&[3, 4]), neighbors);
    }

    #[test]
    fn test_ordered_ranges() {
        // this will be a graph defined by the subset relation
        let mut graph = Graph::new_ordered_from_slice(&[
            Word::new([0, 1]),
            Word::new([0]),
            Word::new([1]),
            Word::new([2, 3]),
            Word::new([1, 2, 3]),
            Word::new([4]),
        ]);

        assert_eq!(
            graph.values,
            vec![
                Word::new([0]),
                Word::new([1]),
                Word::new([4]),
                Word::new([0, 1]),
                Word::new([2, 3]),
                Word::new([1, 2, 3]),
            ]
        );

        graph.push_edge(0, 3);
        graph.push_edge(1, 3);
        graph.push_edge(1, 5);
        graph.push_edge(4, 5);

        let mut ranges = HashMap::new();

        ranges.insert(1, vec![0, 1]);
        ranges.insert(2, vec![3, 4]);
        ranges.insert(3, vec![5]);

        assert_eq!(graph.word_size_ranges(), ranges);
    }

    #[test]
    fn test_clique() {
        let mut graph = Graph::default();

        graph.push("a");
        graph.push("b");
        graph.push("c");
        graph.push("d");

        graph.push_edge(0, 1);
        graph.push_edge(0, 2);
        graph.push_edge(1, 2);

        let mut rng = rand::rng();

        assert_eq!(graph.sample_clique(0, &mut rng), vec![0, 1, 2]);
        assert_eq!(graph.sample_clique(1, &mut rng), vec![0, 1, 2]);
        assert_eq!(graph.sample_clique(2, &mut rng), vec![0, 1, 2]);
        assert_eq!(graph.sample_clique(3, &mut rng), vec![3]);
    }
}
