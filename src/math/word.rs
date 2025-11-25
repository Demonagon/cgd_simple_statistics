use std::fmt::Display;

use smallvec::SmallVec;

use rand::Rng;

/// A word with alphabet size up to 256.
/// Meant for both words in substitutions and configurations.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Word {
    pub data: SmallVec<[u8; 8]>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.data {
            f.write_fmt(format_args!("{l}"))?;
        }

        Ok(())
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.data.len() == other.data.len() {
            return self.data.cmp(&other.data);
        }

        self.data.len().cmp(&other.data.len())
    }
}

impl Word {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn has_prefix(&self, other: &Word) -> bool {
        self.data.strip_prefix(other.data.as_slice()).is_some()
    }

    pub fn new<T: IntoIterator<Item = u8>>(data: T) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }

    pub fn value_sum(&self) -> u32 {
        self.data.iter().map(|&k| k as u32).sum()
    }

    pub fn random<R: Rng>(rng: &mut R, length: u32, alphabet_size: u32) -> Self {
        Self {
            data: rng
                .random_iter::<u8>()
                .map(|k| k % alphabet_size as u8)
                .take(length as usize)
                .collect(),
        }
    }

    pub fn reversed(&self) -> Self {
        Word {
            data: self.data.iter().cloned().rev().collect(),
        }
    }

    pub fn intersects_suffixes_of(&self, other: &Word) -> bool {
        let min_offset = 0.max(other.len() as isize - self.len() as isize) as usize;
        for offset in min_offset..other.len() {
            let comparable_length = (other.len() - offset).min(self.len());
            if comparable_length == 0 {
                continue;
            }

            if self.data[0..comparable_length] == other.data[offset..offset + comparable_length] {
                return true;
            }
        }

        false
    }

    pub fn is_in(&self, other: &Word) -> bool {
        if other.len() < self.len() {
            return false;
        }

        let max_offset = other.len() - self.len();

        for offset in 0..=max_offset {
            if self.data[0..self.len()] == other.data[offset..offset + self.len()] {
                return true;
            }
        }

        false
    }

    pub fn is_bifix_free_with(&self, other: &Word) -> bool {
        let self_size = self.data.len();
        let other_size = other.data.len();

        for offset in -(self_size as isize - 1)..other_size as isize {
            if offset == 0 && self_size == other_size {
                continue;
            }

            let self_offset = 0.max(-offset) as usize;
            let other_offset = 0.max(offset) as usize;

            let len = (self_size - self_offset).min(other_size - other_offset);

            if self.data[self_offset..self_offset + len]
                == other.data[other_offset..other_offset + len]
            {
                return false;
            }
        }

        true
    }

    pub fn is_self_bifix_free(&self) -> bool {
        self.is_bifix_free_with(self)
    }

    pub fn shared_prefix(&self, other: &Self) -> Self {
        let mut result = Word::new([]);

        for k in 0..self.len().min(other.len()) {
            if self.data[k] == other.data[k] {
                result.data.push(self.data[k]);
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn display() {
        let word = Word {
            data: smallvec![0, 1, 2, 3],
        };

        assert_eq!("0123", format!("{word}"));
    }

    #[test]
    fn prefix() {
        let a = Word::new([0, 2, 0, 1]);
        let b = Word::new([0, 2]);

        assert!(a.has_prefix(&b));
        assert!(!b.has_prefix(&a));
        assert!(a.has_prefix(&a));
        assert!(b.has_prefix(&b));
    }

    #[test]
    fn inclusion() {
        assert!(!Word::new([0, 1]).is_bifix_free_with(&Word::new([2, 0, 1, 3])))
    }

    #[test]
    fn prefix_suffix() {
        assert!(Word::new([0, 1, 2]).intersects_suffixes_of(&Word::new([1, 2, 0])));
        assert!(Word::new([1, 2, 0]).intersects_suffixes_of(&Word::new([0, 1, 2])));
        assert!(Word::new([1, 2, 0]).intersects_suffixes_of(&Word::new([1, 2, 0])));
        assert!(!Word::new([1, 2, 0]).intersects_suffixes_of(&Word::new([2, 2, 0])));
        assert!(!Word::new([0]).intersects_suffixes_of(&Word::new([1, 0, 1])));
        assert!(Word::new([0]).intersects_suffixes_of(&Word::new([1, 0])));
        assert!(Word::new([0]).intersects_suffixes_of(&Word::new([1, 0])));
        assert!(Word::new([0]).intersects_suffixes_of(&Word::new([1, 1, 1, 0])));
        assert!(!Word::new([1, 1, 1, 0]).intersects_suffixes_of(&Word::new([0])));
        assert!(!Word::new([0]).intersects_suffixes_of(&Word::new([0, 1])));
    }

    #[test]
    fn bifix_free() {
        assert!(
            Word {
                data: smallvec![0, 0]
            }
            .is_bifix_free_with(&Word {
                data: smallvec![1, 1]
            })
        );
        assert!(
            !Word {
                data: smallvec![0, 1]
            }
            .is_bifix_free_with(&Word {
                data: smallvec![1, 1]
            })
        );
        assert!(
            Word {
                data: smallvec![1, 0, 1, 0, 0]
            }
            .is_bifix_free_with(&Word {
                data: smallvec![1, 1, 0, 0, 0,]
            })
        );
    }

    #[test]
    fn self_bifix_free() {
        assert!(Word { data: smallvec![0] }.is_self_bifix_free());
        assert!(Word { data: smallvec![1] }.is_self_bifix_free());

        assert!(
            !Word {
                data: smallvec![0, 0]
            }
            .is_self_bifix_free()
        );
        assert!(
            Word {
                data: smallvec![0, 1]
            }
            .is_self_bifix_free()
        );
        assert!(
            Word {
                data: smallvec![1, 0]
            }
            .is_self_bifix_free()
        );
        assert!(
            !Word {
                data: smallvec![1, 1]
            }
            .is_self_bifix_free()
        );

        assert!(
            Word {
                data: smallvec![1, 0, 1, 0, 0]
            }
            .is_self_bifix_free()
        );
        assert!(
            Word {
                data: smallvec![1, 1, 0, 0, 0]
            }
            .is_self_bifix_free()
        );

        assert!(
            !Word {
                data: smallvec![1, 1, 0, 0, 0, 1, 1, 0, 0, 0]
            }
            .is_self_bifix_free()
        );
    }

    #[test]
    fn test_shared_prefix() {
        assert_eq!(
            Word::new([0, 1, 2]).shared_prefix(&Word::new([0, 1])),
            Word::new([0, 1])
        );

        assert_eq!(
            Word::new([0, 5, 4, 3, 0, 1]).shared_prefix(&Word::new([0, 5, 4, 2, 1, 4])),
            Word::new([0, 5, 4])
        );

        assert_eq!(
            Word::new([0, 1, 2]).shared_prefix(&Word::new([2])),
            Word::new([])
        );
    }

    #[test]
    fn test_is_in() {
        assert!(Word::new([0, 1, 1]).is_in(&Word::new([0, 1, 1])));
        assert!(Word::new([0, 1]).is_in(&Word::new([0, 1, 1])));
        assert!(Word::new([0]).is_in(&Word::new([0, 1, 1])));
        assert!(Word::new([1]).is_in(&Word::new([0, 1, 1])));
        assert!(Word::new([]).is_in(&Word::new([0, 1, 1])));
        assert!(Word::new([]).is_in(&Word::new([])));
        assert!(!Word::new([1]).is_in(&Word::new([])));
        assert!(!Word::new([0, 1, 1]).is_in(&Word::new([0, 1])));
    }
}
