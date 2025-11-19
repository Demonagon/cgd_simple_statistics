use std::num::NonZeroU32;

use crate::math::word::Word;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<const ALPHABET_SIZE: usize> {
    included: bool,
    prefix: Vec<u8>,
    children: [Option<NonZeroU32>; ALPHABET_SIZE],
}

#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct Tree<const ALPHABET_SIZE: usize> {
    nodes: Vec<Node<ALPHABET_SIZE>>,
}

impl<const ALPHABET_SIZE: usize> Tree<ALPHABET_SIZE> {
    fn insert_at(&mut self, index: usize, word: &[u8]) {
        match word.strip_prefix(self.nodes[index].prefix.as_slice()) {
            Some(suffix) => {
                // the word fits the node

                let Some(initial) = suffix.first() else {
                    // word is at this exact node
                    self.nodes[index].included = true;
                    return;
                };

                match self.nodes[index].children[*initial as usize] {
                    Some(child_index) => {
                        // we continue our exploration
                        self.insert_at(child_index.get() as usize, suffix);
                    }
                    None => {
                        // we reserve a new index
                        let new_index = self.nodes.len();

                        // we insert the new index
                        self.nodes[index].children[*initial as usize] = Some(
                            NonZeroU32::new(new_index as u32)
                                .expect("len() can't be 0 if the match block doesn't panic"),
                        );

                        // we create a new node
                        self.nodes.push(Node {
                            included: true,
                            prefix: Vec::from(suffix),
                            children: [None; ALPHABET_SIZE],
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
                        let &node_first_letter = node_suffix.first().unwrap();
                        let word_suffix = word.strip_prefix(shared_prefix.data.as_slice()).unwrap();
                        let &word_first_letter = word_suffix.first().unwrap();

                        // we reserve indexes for new nodes
                        let new_node_index = self.nodes.len();
                        let new_word_index = self.nodes.len() + 1;

                        // we push the new indices
                        self.nodes.push(Node {
                            included: true,
                            prefix: Vec::from(node_suffix),
                            children: self.nodes[index].children,
                        });
                        self.nodes.push(Node {
                            included: true,
                            prefix: Vec::from(word_suffix),
                            children: [None; ALPHABET_SIZE],
                        });

                        // the current node will now contain that shared prefix
                        self.nodes[index].prefix = Vec::from(shared_prefix.data.as_slice());
                        // it is not a word itself
                        self.nodes[index].included = false;
                        self.nodes[index].children = [None; ALPHABET_SIZE];
                        self.nodes[index].children[word_first_letter as usize] =
                            Some(NonZeroU32::new(new_word_index as u32).unwrap());
                        self.nodes[index].children[node_first_letter as usize] =
                            Some(NonZeroU32::new(new_node_index as u32).unwrap());

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
                    included: true,
                    prefix: suffix,
                    children: self.nodes[index].children,
                });

                // we remove all the moved children
                self.nodes[index].children = [None; ALPHABET_SIZE];

                // we insert the new index
                self.nodes[index].children[initial as usize] = Some(
                    NonZeroU32::new(new_index as u32)
                        .expect("len() can't be 0 if the match block doesn't panic"),
                );
            }
        };
    }

    pub fn insert(&mut self, word: Word) {
        if self.nodes.is_empty() {
            self.nodes.push(Node {
                included: true,
                prefix: Vec::from(word.data.as_slice()),
                children: [None; ALPHABET_SIZE],
            });
        } else {
            self.insert_at(0, &word.data);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeIterator<'a, const ALPHABET_SIZE: usize> {
    tree: &'a Tree<ALPHABET_SIZE>,
    current_word: Word,
    current_index: usize,
}

impl<'a, const ALPHABET_SIZE: usize> Iterator for TreeIterator<'a, ALPHABET_SIZE> {
    type Item = &'a Word;

    fn next(&mut self) -> Option<Self::Item> {
        // we have to walk the tree...
        // we need to keep track of the next path somehow
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(k: u32) -> Option<NonZeroU32> {
        Some(NonZeroU32::new(k).unwrap())
    }

    #[test]
    fn tree_creation() {
        let mut tree = Tree::<3>::default();

        assert_eq!(tree, Tree { nodes: vec![] });

        tree.insert(Word::new([0]));

        assert_eq!(
            tree,
            Tree {
                nodes: vec![Node {
                    included: true,
                    prefix: vec![0],
                    children: [None; 3],
                }]
            }
        );

        tree.insert(Word::new([0, 1]));

        assert_eq!(
            tree,
            Tree {
                nodes: vec![
                    Node {
                        included: true,
                        prefix: vec![0],
                        children: [None, index(1), None],
                    },
                    Node {
                        included: true,
                        prefix: vec![1],
                        children: [None; 3],
                    }
                ]
            }
        );

        tree.insert(Word::new([2, 0]));

        assert_eq!(
            tree,
            Tree {
                nodes: vec![
                    Node {
                        included: false,
                        prefix: vec![],
                        children: [index(2), None, index(3)],
                    },
                    Node {
                        included: true,
                        prefix: vec![1],
                        children: [None; 3],
                    },
                    Node {
                        included: true,
                        prefix: vec![0],
                        children: [None, index(1), None],
                    },
                    Node {
                        included: true,
                        prefix: vec![2, 0],
                        children: [None; 3],
                    },
                ]
            }
        );

        tree.insert(Word::new([0, 1, 2]));

        assert_eq!(
            tree,
            Tree {
                nodes: vec![
                    Node {
                        included: false,
                        prefix: vec![],
                        children: [index(2), None, index(3)],
                    },
                    Node {
                        included: true,
                        prefix: vec![1],
                        children: [None, None, index(4)],
                    },
                    Node {
                        included: true,
                        prefix: vec![0],
                        children: [None, index(1), None],
                    },
                    Node {
                        included: true,
                        prefix: vec![2, 0],
                        children: [None; 3],
                    },
                    Node {
                        included: true,
                        prefix: vec![2],
                        children: [None; 3],
                    },
                ]
            }
        );
    }
}
