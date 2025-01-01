use std::collections::HashMap;

pub fn quick_sort(list: &mut Vec<i64>) {
    quick_sort_recursive(list, 0, list.len() - 1);

    fn quick_sort_recursive(list: &mut Vec<i64>, low_idx: usize, high_idx: usize) {
        fn swap(list: &mut Vec<i64>, idx_one: usize, idx_two: usize) {
            let temp = list[idx_one];
            list[idx_one] = list[idx_two];
            list[idx_two] = temp;
        }

        if high_idx <= low_idx {
            return;
        }

        let mut pivot_idx = low_idx + (high_idx - low_idx) / 2;
        let pivot = list[pivot_idx];
        let mut left_idx = low_idx;
        let mut right_idx = high_idx;

        while left_idx <= right_idx && right_idx > 0 {
            if list[left_idx] <= pivot {
                left_idx += 1;
            } else if list[right_idx] >= pivot {
                right_idx -= 1;
            } else {
                swap(list, left_idx, right_idx);
                left_idx += 1;
                right_idx -= 1;
            }
        }

        if pivot_idx < right_idx {
            swap(list, pivot_idx, right_idx);
            pivot_idx = right_idx;
        } else if pivot_idx > left_idx {
            swap(list, pivot_idx, left_idx);
            pivot_idx = left_idx;
        }

        if pivot_idx > 0 {
            quick_sort_recursive(list, low_idx, pivot_idx - 1);
        }
        quick_sort_recursive(list, pivot_idx + 1, high_idx);
    }
}

pub fn calculate_distance(list_one: &Vec<i64>, list_two: &Vec<i64>) -> Result<i64, &'static str> {
    let mut distance: i64 = 0;

    if list_one.len() != list_two.len() {
        return Err("Both lists should have same length");
    }

    for it in list_one.iter().zip(list_two.iter()) {
        distance += i64::abs(it.0 - it.1);
    }

    Ok(distance)
}

pub fn calculate_similarity(list_one: &Vec<i64>, list_two: &Vec<i64>) -> Result<i64, &'static str> {
    let mut similarity: i64 = 0;
    let mut list_two_map: HashMap<i64, i64> = HashMap::new();

    for num in list_two {
        match list_two_map.get(num) {
            Some(count) => list_two_map.insert(*num, count + 1),
            None => list_two_map.insert(*num, 1),
        };
    }

    for num in list_one {
        if let Some(count) = list_two_map.get(num) {
            similarity += *count * *num;
        }
    }

    Ok(similarity)
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
        quick_sort(&mut list_copy);

        list.sort();

        for idx in 0..list.len() {
            assert_eq!(list[idx], list_copy[idx]);
        }
    }
}
