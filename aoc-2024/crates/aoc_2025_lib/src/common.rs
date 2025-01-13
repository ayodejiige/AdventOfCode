use std::cmp::Ordering;

fn quick_sort_recursive<T, F>(list: &mut Vec<T>, low_idx: usize, high_idx: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    if high_idx <= low_idx {
        return;
    }

    let mut pivot_idx = low_idx + (high_idx - low_idx) / 2;
    let mut left_idx = low_idx;
    let mut right_idx = high_idx;

    while left_idx <= right_idx && right_idx > 0 {
        let left_compare = compare(&list[left_idx], &list[pivot_idx]);
        let right_compare = compare(&list[right_idx], &list[pivot_idx]);
        if left_compare.is_le() {
            left_idx += 1;
        } else if right_compare.is_ge() {
            right_idx -= 1;
        } else {
            list.swap(left_idx, right_idx);
            left_idx += 1;
            right_idx -= 1;
        }
    }

    if pivot_idx < right_idx {
        list.swap(pivot_idx, right_idx);
        pivot_idx = right_idx;
    } else if pivot_idx > left_idx {
        list.swap(pivot_idx, left_idx);
        pivot_idx = left_idx;
    }

    if pivot_idx > 0 {
        quick_sort_recursive(list, low_idx, pivot_idx - 1, compare);
    }
    quick_sort_recursive(list, pivot_idx + 1, high_idx, compare);
}

pub fn quick_sort<T, F>(list: &mut Vec<T>, compare: F) 
where
    F: Fn(&T, &T) -> Ordering,
{
    quick_sort_recursive(list, 0, list.len() - 1, &compare);
}

/// Struct to represent a position in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Create a new instance of the position.
    ///
    /// # Arguments
    ///    row: The row index of the position.
    ///    col: The column index of the position.
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn test_sorting() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 100000);

        let mut list = (0..10000).map(|_| rng.sample(&range)).collect::<Vec<i64>>();
        let mut list_copy = list.clone();
        quick_sort(&mut list_copy, |a, b| a.cmp(b));

        list.sort();

        for idx in 0..list.len() {
            assert_eq!(list[idx], list_copy[idx]);
        }
    }
}
