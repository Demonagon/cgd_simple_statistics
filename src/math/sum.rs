//! This module contains code related to enumerating arrays of integers
//! that sum to the same value,
//! and permutations of those sums.
//! The tricky part is that those permutations
//! shouldn't repeat, e.g. 1, 2, 1 should only
//! be counted once

pub fn enumerate_sorted_sums(sum: u32) -> Vec<Vec<u32>> {
    if sum == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];

    let mut current_array = vec![1];
    let mut remaining = sum - 1;

    loop {
        // we found a list
        if remaining == 0 {
            result.push(current_array.clone());
            // we remove the last digit
            // safety : loop condition ensures this unwraps
            remaining += current_array.pop().unwrap();

            // and increment the last last digit
            if let Some(previous) = current_array.last_mut() {
                *previous += 1;
                remaining -= 1;
            } else {
                // if we can't, we finished
                break;
            }
        } else {
            current_array.push(1);
            remaining -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_sorted_sums() {
        let sums = enumerate_sorted_sums(4);

        assert_eq!(sums.len(), 8);

        assert_eq!(sums[0], vec![1, 1, 1, 1]);
        assert_eq!(sums[1], vec![1, 1, 2]);
        assert_eq!(sums[2], vec![1, 2, 1]);
        assert_eq!(sums[3], vec![1, 3]);
        assert_eq!(sums[4], vec![2, 1, 1]);
        assert_eq!(sums[5], vec![2, 2]);
        assert_eq!(sums[6], vec![3, 1]);
        assert_eq!(sums[7], vec![4]);
    }
}
