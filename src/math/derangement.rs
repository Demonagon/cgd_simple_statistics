pub fn factorial(k: u32) -> u32 {
    if k <= 1 { 1 } else { k * factorial(k - 1) }
}

fn permutation(k: u32, write_to: &mut [usize]) {
    let size = write_to.len() as u32;

    if size == 0 {
        return;
    }

    if size == 1 {
        write_to[0] = 0;
        return;
    }

    let head = k.rem_euclid(size) as usize;

    permutation(k.div_euclid(size), &mut write_to[1..]);

    write_to[0] = head;
    write_to[1..].iter_mut().for_each(|v| {
        if *v >= head {
            *v += 1
        }
    });
}

pub fn permutations(size: u32) -> Vec<Vec<usize>> {
    if size == 0 {
        return vec![vec![]];
    }

    let total = factorial(size);

    (0..total)
        .map(move |k| {
            let mut result = vec![0; size as usize];
            permutation(k, &mut result);
            result
        })
        .collect()
}

/// Derangements are permutations with no fixed points
pub fn derangements(size: u32) -> Vec<Vec<usize>> {
    permutations(size)
        .into_iter()
        .filter(|p| p.iter().enumerate().all(|(i, k)| i != *k))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(2), vec![vec![0, 1], vec![1, 0],]);
        assert_eq!(
            permutations(3),
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ]
        );
        assert_eq!(
            permutations(4),
            vec![
                vec![0, 1, 2, 3],
                vec![1, 0, 2, 3],
                vec![2, 0, 1, 3],
                vec![3, 0, 1, 2],
                vec![0, 2, 1, 3],
                vec![1, 2, 0, 3],
                vec![2, 1, 0, 3],
                vec![3, 1, 0, 2],
                vec![0, 3, 1, 2],
                vec![1, 3, 0, 2],
                vec![2, 3, 0, 1],
                vec![3, 2, 0, 1],
                vec![0, 1, 3, 2],
                vec![1, 0, 3, 2],
                vec![2, 0, 3, 1],
                vec![3, 0, 2, 1],
                vec![0, 2, 3, 1],
                vec![1, 2, 3, 0],
                vec![2, 1, 3, 0],
                vec![3, 1, 2, 0],
                vec![0, 3, 2, 1],
                vec![1, 3, 2, 0],
                vec![2, 3, 1, 0],
                vec![3, 2, 1, 0]
            ]
        );
    }

    #[test]
    fn test_derangements() {
        assert_eq!(derangements(2), vec![vec![1, 0],]);
        assert_eq!(derangements(3), vec![vec![2, 0, 1], vec![1, 2, 0],]);
        assert_eq!(
            derangements(4),
            vec![
                vec![3, 0, 1, 2],
                vec![1, 3, 0, 2],
                vec![2, 3, 0, 1],
                vec![3, 2, 0, 1],
                vec![1, 0, 3, 2],
                vec![2, 0, 3, 1],
                vec![1, 2, 3, 0],
                vec![2, 3, 1, 0],
                vec![3, 2, 1, 0]
            ]
        );
    }
}
