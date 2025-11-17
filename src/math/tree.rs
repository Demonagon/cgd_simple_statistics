use std::num::NonZeroU32;

use crate::math::word::Word;

#[derive(Clone)]
pub struct Node<const ALPHABET_SIZE: usize> {
    prefix: Vec<u8>,
    children: [Option<NonZeroU32>; ALPHABET_SIZE],
}

#[derive(Default, Clone)]
pub struct Tree<const ALPHABET_SIZE: usize> {
    nodes: Vec<Node<ALPHABET_SIZE>>,
}

impl<const ALPHABET_SIZE: usize> Tree<ALPHABET_SIZE> {
    fn insert_at(&mut self, index: usize, word: &[u8]) {
        match word.strip_prefix(self.nodes[index].prefix.as_slice()) {
            Some(suffix) => {
                let Some(initial) = suffix.first() else {
                    // word is already contained
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
                            prefix: Vec::from(suffix),
                            children: [None; ALPHABET_SIZE],
                        })
                    }
                }
            }
            None => {
                // and move the previous node
                // to a new place, with the same children

                let suffix = {
                    let Some(suffix) = self.nodes[index].prefix.strip_prefix(word) else {
                        // words are equal, or incomparable, which shoudln't happen
                        return;
                    };
                    Vec::from(suffix)
                };

                // so the word to insert is a prefix to the current word
                // we insert the current prefix
                self.nodes[index].prefix = Vec::from(word);

                // we reserve a new index
                let new_index = self.nodes.len();

                let Some(&initial) = suffix.first() else {
                    // word is already contained
                    return;
                };

                // we create a new node
                self.nodes.push(Node {
                    prefix: suffix,
                    children: self.nodes[index].children,
                });

                // we insert the new index
                self.nodes[index].children[initial as usize] = Some(
                    NonZeroU32::new(new_index as u32)
                        .expect("len() can't be 0 if the match block doesn't panic"),
                );
            }
        };
    }

    pub fn insert(&mut self, word: Word) {
        self.insert_at(0, &word.data);
    }
}
