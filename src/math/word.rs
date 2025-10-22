use std::fmt::Display;

/// A word with alphabet size up to 256.
/// Meant for both words in substitutions and configurations.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Word {
    pub data: Vec<u8>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.data {
            f.write_fmt(format_args!("{l}"))?;
        }

        Ok(())
    }
}

impl Word {
    pub fn new<T: IntoIterator<Item = u8>>(data: T) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }

    pub fn is_bifix_free_with(&self, other: &Word) -> bool {
        let self_size = self.data.len();
        let other_size = other.data.len();

        for offset in -(self_size as isize - 1)..other_size as isize {
            if offset == 0 && self_size == other_size {
                continue;
            }

            let self_offset = 0.max(offset) as usize;
            let other_offset = 0.max(-offset) as usize;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let word = Word {
            data: vec![0, 1, 2, 3],
        };

        assert_eq!("0123", format!("{word}"));
    }

    #[test]
    fn bifix_free() {
        assert!(Word { data: vec![0, 0] }.is_bifix_free_with(&Word { data: vec![1, 1] }));
        assert!(!Word { data: vec![0, 1] }.is_bifix_free_with(&Word { data: vec![1, 1] }));
        assert!(
            Word {
                data: vec![1, 0, 1, 0, 0]
            }
            .is_bifix_free_with(&Word {
                data: vec![1, 1, 0, 0, 0,]
            })
        );
    }

    #[test]
    fn self_bifix_free() {
        assert!(Word { data: vec![0] }.is_self_bifix_free());
        assert!(Word { data: vec![1] }.is_self_bifix_free());

        assert!(!Word { data: vec![0, 0] }.is_self_bifix_free());
        assert!(Word { data: vec![0, 1] }.is_self_bifix_free());
        assert!(Word { data: vec![1, 0] }.is_self_bifix_free());
        assert!(!Word { data: vec![1, 1] }.is_self_bifix_free());

        assert!(
            Word {
                data: vec![1, 0, 1, 0, 0]
            }
            .is_self_bifix_free()
        );
        assert!(
            Word {
                data: vec![1, 1, 0, 0, 0]
            }
            .is_self_bifix_free()
        );

        assert!(
            !Word {
                data: vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 0]
            }
            .is_self_bifix_free()
        );
    }
}
