use smallvec::{SmallVec, smallvec};

use crate::math::word::Word;

const NODE_CAPACITY: usize = 8;

// ###############################################################
// #                                                             #
// #                     NODE                                    #
// #                                                             #
// ###############################################################

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    // if this nodes corresponds to a word,
    // contains its given index
    pub word_index: Option<usize>,
    pub prefix: Vec<u8>,
    pub children: SmallVec<[Option<usize>; NODE_CAPACITY]>,
}

impl Node {
    fn first_child(&self) -> Option<usize> {
        for child in &self.children {
            if child.is_some() {
                return *child;
            }
        }

        None
    }
    fn next_child_after(&self, previous: u8) -> Option<usize> {
        for k in (previous as usize + 1)..self.children.len() {
            let child = self.children[k];

            if child.is_some() {
                return child;
            }
        }

        None
    }
}

// ###############################################################
// #                                                             #
// #                     TREE                                    #
// #                                                             #
// ###############################################################

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tree {
    pub alphabet_size: usize,
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new(alphabet_size: usize) -> Self {
        Self {
            alphabet_size,
            nodes: vec![],
        }
    }

    fn insert_at(&mut self, index: usize, word_index: usize, word: &[u8]) {
        match word.strip_prefix(self.nodes[index].prefix.as_slice()) {
            Some(suffix) => {
                // the word fits the node

                let Some(initial) = suffix.first() else {
                    // word is at this exact node
                    self.nodes[index].word_index = Some(word_index);
                    return;
                };

                match self.nodes[index].children[*initial as usize] {
                    Some(child_index) => {
                        // we continue our exploration
                        self.insert_at(child_index, word_index, suffix);
                    }
                    None => {
                        // we reserve a new index
                        let new_index = self.nodes.len();

                        // we insert the new index
                        self.nodes[index].children[*initial as usize] = Some(new_index);

                        // we create a new node
                        self.nodes.push(Node {
                            word_index: Some(word_index),
                            prefix: Vec::from(suffix),
                            children: smallvec![None; self.alphabet_size],
                        })
                    }
                }
            }
            None => {
                // the word doesn't fit the node; it means we are a prefix of it
                // so we move the node
                // to a new place, with the same children

                let suffix = {
                    let Some(suffix) = self.nodes[index].prefix.strip_prefix(word) else {
                        // words are incomparable, so we need to create a new node
                        // for the shared prefix
                        let shared_prefix = Word::new(self.nodes[index].prefix.clone())
                            .shared_prefix(&Word::new(word.to_vec()));

                        let node_suffix = self.nodes[index]
                            .prefix
                            .strip_prefix(shared_prefix.data.as_slice())
                            .unwrap();
                        let node_word_index = self.nodes[index].word_index;
                        let &node_first_letter = node_suffix.first().unwrap();
                        let word_suffix = word.strip_prefix(shared_prefix.data.as_slice()).unwrap();
                        let &word_first_letter = word_suffix.first().unwrap();

                        // we reserve indexes for new nodes
                        let new_node_index = self.nodes.len();
                        let new_word_index = self.nodes.len() + 1;

                        // we push the new indices
                        self.nodes.push(Node {
                            word_index: node_word_index,
                            prefix: Vec::from(node_suffix),
                            children: self.nodes[index].children.clone(),
                        });
                        self.nodes.push(Node {
                            word_index: Some(word_index),
                            prefix: Vec::from(word_suffix),
                            children: smallvec![None; self.alphabet_size],
                        });

                        // the current node will now contain that shared prefix
                        self.nodes[index].prefix = Vec::from(shared_prefix.data.as_slice());
                        // it is not a word itself
                        self.nodes[index].word_index = None;
                        self.nodes[index].children = smallvec![None; self.alphabet_size];
                        self.nodes[index].children[word_first_letter as usize] =
                            Some(new_word_index);
                        self.nodes[index].children[node_first_letter as usize] =
                            Some(new_node_index);

                        return;
                    };
                    Vec::from(suffix)
                };

                // so the word to insert is a prefix to the current word
                // we insert the current prefix
                self.nodes[index].prefix = Vec::from(word);

                // we reserve a new index
                let new_index = self.nodes.len();

                let &initial = suffix
                    .first()
                    .expect("The equal words case is already handled");

                // we create a new node for the word
                self.nodes.push(Node {
                    word_index: self.nodes[index].word_index,
                    prefix: suffix,
                    children: self.nodes[index].children.clone(),
                });

                // we remove all the moved children
                self.nodes[index].children = smallvec![None; self.alphabet_size];

                // we insert the new index
                self.nodes[index].children[initial as usize] = Some(new_index);

                // we insert the word here
                self.nodes[index].word_index = Some(word_index);
            }
        };
    }

    pub fn insert(&mut self, word_index: usize, word: Word) {
        if self.nodes.is_empty() {
            self.nodes.push(Node {
                word_index: Some(word_index),
                prefix: Vec::from(word.data.as_slice()),
                children: smallvec![None; self.alphabet_size],
            });
        } else {
            self.insert_at(0, word_index, &word.data);
        }
    }
}

// ###############################################################
// #                                                             #
// #                   STANDARD TREE ITERATOR                    #
// #                                                             #
// ###############################################################

impl Tree {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = usize> {
        PrefixFilteringTreeIterator::new(self, |_| true)
    }
}

// ###############################################################
// #                                                             #
// #        PREFIX FILTERING TREE WORD INDEX ITERATOR            #
// #                                                             #
// ###############################################################
//
// Does not explore branches of the tree where the provided filter
// returns false for the current word.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TreeIteratorState {
    Up,
    Down(u8),
    End,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixFilteringTreeIterator<'a, F: Fn(&Word) -> bool> {
    tree: &'a Tree,
    filter_closure: F,
    current_word: Word,
    index_stack: Vec<usize>,
    state: TreeIteratorState,
}

impl<'a, F: Fn(&Word) -> bool> PrefixFilteringTreeIterator<'a, F> {
    fn new(tree: &'a Tree, filter: F) -> Self {
        Self {
            tree,
            filter_closure: filter,
            current_word: Word::default(),
            index_stack: vec![0],
            state: TreeIteratorState::Up,
        }
    }

    fn walk_to_next_word(&mut self) -> Option<usize> {
        if self.tree.nodes.is_empty() {
            self.state = TreeIteratorState::End;
            return None;
        }

        loop {
            match self.state {
                TreeIteratorState::Up => {
                    let &current_index = self
                        .index_stack
                        .last()
                        .expect("No current index going up in tree iterator");

                    self.current_word
                        .data
                        .extend(self.tree.nodes[current_index].prefix.iter().copied());

                    if !(self.filter_closure)(&self.current_word) {
                        match self.tree.nodes[current_index].prefix.first() {
                            Some(&letter) => {
                                self.state = TreeIteratorState::Down(letter);
                            }
                            None => {
                                // Empty prefix is possible at the root
                                // in which case the process ends
                                self.state = TreeIteratorState::End;
                            }
                        };

                        continue;
                    }

                    match self.tree.nodes[current_index].first_child() {
                        Some(index) => {
                            self.state = TreeIteratorState::Up;
                            self.index_stack.push(index);
                        }
                        None => {
                            match self.tree.nodes[current_index].prefix.first() {
                                Some(&letter) => {
                                    self.state = TreeIteratorState::Down(letter);
                                }
                                None => {
                                    // Empty prefix is possible at the root
                                    // in which case the process ends
                                    self.state = TreeIteratorState::End;
                                }
                            };
                        }
                    }

                    if self.tree.nodes[current_index].word_index.is_some() {
                        return self.tree.nodes[current_index].word_index;
                    }
                }
                TreeIteratorState::Down(letter) => {
                    let child_index = self
                        .index_stack
                        .pop()
                        .expect("Going down with no index in stack");

                    let Some(&parent_index) = self.index_stack.last() else {
                        // No parent, implies we are trying to go down
                        // from the root.
                        self.state = TreeIteratorState::End;
                        return None;
                    };

                    self.current_word.data = self
                        .current_word
                        .data
                        .strip_suffix(self.tree.nodes[child_index].prefix.as_slice())
                        .expect("Removing child prefix wasnt possible")
                        .into();

                    match self.tree.nodes[parent_index].next_child_after(letter) {
                        Some(next_child_index) => {
                            self.index_stack.push(next_child_index);
                            self.state = TreeIteratorState::Up;
                        }
                        None => {
                            // we chain down
                            match self.tree.nodes[parent_index].prefix.first() {
                                Some(&letter) => {
                                    self.state = TreeIteratorState::Down(letter);
                                }
                                None => {
                                    // Empty prefix is possible at the root
                                    // in which case the process ends
                                    self.state = TreeIteratorState::End;
                                    return None;
                                }
                            };
                        }
                    };
                }
                TreeIteratorState::End => {
                    return None;
                }
            }
        }
    }
}

impl<'a, F: Fn(&Word) -> bool> Iterator for PrefixFilteringTreeIterator<'a, F> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.walk_to_next_word()
    }
}

// ###############################################################
// #                                                             #
// #                     PREFIX-FREE ITERATION                   #
// #                                                             #
// ###############################################################

impl Tree {
    /// Iterators over words which do not have a prefix that is a suffix of
    /// a word given in words.
    pub fn prefix_free_iter<'a>(&'a self, words: &'a [Word]) -> impl Iterator<Item = usize> + 'a {
        PrefixFilteringTreeIterator::new(self, |word| {
            words.iter().all(|w| !w.intersects_suffixes_of(word))
        })
    }
}

// ###############################################################
// #                                                             #
// #                        TESTS                                #
// #                                                             #
// ###############################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_creation() {
        let mut tree = Tree::new(3);

        assert_eq!(
            tree,
            Tree {
                alphabet_size: 3,
                nodes: vec![]
            }
        );

        tree.insert(0, Word::new([0]));

        assert_eq!(
            tree,
            Tree {
                alphabet_size: 3,
                nodes: vec![Node {
                    word_index: Some(0),
                    prefix: vec![0],
                    children: smallvec![None; 3],
                }]
            }
        );

        tree.insert(1, Word::new([0, 1]));

        assert_eq!(
            tree,
            Tree {
                alphabet_size: 3,
                nodes: vec![
                    Node {
                        word_index: Some(0),
                        prefix: vec![0],
                        children: smallvec![None, Some(1), None],
                    },
                    Node {
                        word_index: Some(1),
                        prefix: vec![1],
                        children: smallvec![None; 3],
                    }
                ]
            }
        );

        tree.insert(3, Word::new([2, 0]));

        assert_eq!(
            tree,
            Tree {
                alphabet_size: 3,
                nodes: vec![
                    Node {
                        word_index: None,
                        prefix: vec![],
                        children: smallvec![Some(2), None, Some(3)],
                    },
                    Node {
                        word_index: Some(1),
                        prefix: vec![1],
                        children: smallvec![None; 3],
                    },
                    Node {
                        word_index: Some(0),
                        prefix: vec![0],
                        children: smallvec![None, Some(1), None],
                    },
                    Node {
                        word_index: Some(3),
                        prefix: vec![2, 0],
                        children: smallvec![None; 3],
                    },
                ]
            }
        );

        tree.insert(2, Word::new([0, 1, 2]));

        assert_eq!(
            tree,
            Tree {
                alphabet_size: 3,
                nodes: vec![
                    Node {
                        word_index: None,
                        prefix: vec![],
                        children: smallvec![Some(2), None, Some(3)],
                    },
                    Node {
                        word_index: Some(1),
                        prefix: vec![1],
                        children: smallvec![None, None, Some(4)],
                    },
                    Node {
                        word_index: Some(0),
                        prefix: vec![0],
                        children: smallvec![None, Some(1), None],
                    },
                    Node {
                        word_index: Some(3),
                        prefix: vec![2, 0],
                        children: smallvec![None; 3],
                    },
                    Node {
                        word_index: Some(2),
                        prefix: vec![2],
                        children: smallvec![None; 3],
                    },
                ]
            }
        );
    }

    #[test]
    fn tree_iterator() {
        let mut tree = Tree::new(3);

        tree.insert(0, Word::new([0]));
        tree.insert(3, Word::new([2, 0]));
        tree.insert(2, Word::new([0, 1, 2]));
        tree.insert(1, Word::new([0, 1]));

        let mut iterator = tree.iter();

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn tree_prefix_filtering_iterator() {
        let mut tree = Tree::new(3);

        tree.insert(0, Word::new([0]));
        tree.insert(3, Word::new([2, 0]));
        tree.insert(2, Word::new([0, 1, 2]));
        tree.insert(1, Word::new([0, 1]));

        let mut iterator = PrefixFilteringTreeIterator::new(&tree, |word| !word.data.contains(&1));

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn prefix_free_iterator() {
        let mut tree = Tree::new(3);

        tree.insert(0, Word::new([0]));
        tree.insert(3, Word::new([2, 0]));
        tree.insert(2, Word::new([0, 1, 2]));
        tree.insert(1, Word::new([0, 1]));

        let words = [Word::new([1])];
        let mut iterator = tree.prefix_free_iter(&words);

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);

        let words = [Word::new([0])];
        let mut iterator = tree.prefix_free_iter(&words);

        assert_eq!(iterator.next(), None);

        let words = [Word::new([2])];
        let mut iterator = tree.prefix_free_iter(&words);

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);

        let words = [Word::new([1, 1, 1, 0])];
        let mut iterator = tree.prefix_free_iter(&words);

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), None);

        let words = [Word::new([0, 0, 1]), Word::new([2, 2, 2, 1, 2])];
        let mut iterator = tree.prefix_free_iter(&words);

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), None);
    }
}
