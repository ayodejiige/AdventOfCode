/// Solution to AdventOfCode 2024 Day 11 Problem - Plutonian Pebbles
///
/// https://adventofcode.com/2024/day/11
use crate::common::number_of_digits;
use std::{collections::HashMap, fs};

/// Enum representing the result of transforming a stone.
enum TransformationResult {
    /// Represents transformations that result in a single stone.
    Single(u64),
    /// Represents transformations that result in a two stones.
    Split(u64, u64),
}

/// Transform a stone based on the following rules.
///
/// - If the stone is 0, it transforms into a single stone with value 1.
/// - If the stone has an even number of digits, it splits into two stones:
///   - The left stone is the first half of the digits.
///   - The right stone is the second half of the digits.
/// - If the stone has an odd number of digits, it transforms into a single stone
///   with value equal to the original stone multiplied by 2024.
fn transform_stone(stone: u64) -> TransformationResult {
    let digit_count = number_of_digits(stone) as u32;
    if stone == 0 {
        TransformationResult::Single(1)
    } else if digit_count % 2 == 0 {
        let left_digit = stone / 10_u64.pow(digit_count / 2);
        let right_digit = stone - (left_digit * 10_u64.pow(digit_count / 2));
        TransformationResult::Split(left_digit, right_digit)
    } else {
        TransformationResult::Single(stone * 2024)
    }
}

/// Brute force approach to simulate the transformation of stones for a given number of blinks.
///
/// For each blink, it transforms each stone and collects the results as the new set of stones. For the next
/// iteration, it uses the new set of stones as the input.
///
/// Returns the total number of stones after the specified number of blinks.
pub fn blink_brute(stones: &Vec<u64>, blinks: usize) -> usize {
    let mut stones = stones.clone();

    for _ in 0..blinks {
        let mut new_stones = Vec::<u64>::new();

        for stone in stones {
            match transform_stone(stone) {
                TransformationResult::Single(new_stone) => {
                    new_stones.push(new_stone);
                }
                TransformationResult::Split(new_stone_1, new_stone_2) => {
                    new_stones.push(new_stone_1);
                    new_stones.push(new_stone_2);
                }
            }
        }

        stones = new_stones;
    }

    stones.len()
}

/// Optimized approach to simulate the transformation of stones using a mapping.
///
/// Instead of keeping track of individual stones, it uses a hashmap to count the occurrences of each stone.
/// For each blink, it transforms the stones in the hashmap and updates the counts accordingly.
///
/// Returns the total number of stones after the specified number of blinks.
pub fn blink_mapped(stones: &Vec<u64>, blinks: usize) -> usize {
    let mut stone_map: HashMap<u64, usize> = HashMap::new();

    // Initialize the stone map with the initial stones.
    for stone in stones {
        stone_map
            .entry(*stone)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut new_stone_map: HashMap<u64, usize> = HashMap::new();

        for (stone, count) in stone_map {
            match transform_stone(stone) {
                TransformationResult::Single(new_stone) => {
                    new_stone_map
                        .entry(new_stone)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
                TransformationResult::Split(stone_1, stone_2) => {
                    new_stone_map
                        .entry(stone_1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    new_stone_map
                        .entry(stone_2)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        }

        stone_map = new_stone_map
    }

    stone_map
        .values()
        .sum()
}

pub fn main(file_path: String) {
    const BLINKS: usize = 75;
    let stones: Vec<u64> = fs::read_to_string(file_path)
        .unwrap()
        .split(" ")
        .map(|s| {
            s.parse()
                .unwrap()
        })
        .collect();

    println!(
        "Stones After {} Blinks: {}",
        BLINKS,
        blink_mapped(&stones, BLINKS)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Input used for unit test.
    const INPUT: [u64; 2] = [125, 17];
    /// Expected results for one to seven blinks.
    const EXPECTED_RESULTS: [usize; 7] = [2, 3, 4, 5, 9, 13, 22];
    /// Exptected result for 25 blinks.
    const EXPECTED_25_BLINKS: usize = 55312;

    /// Helper funct to run tests for one to seven blinks.
    fn run_one_to_seven_blinks<F>(blink_fn: F)
    where
        F: Fn(&Vec<u64>, usize) -> usize,
    {
        let input = INPUT.to_vec();

        for (blinks, &expected) in EXPECTED_RESULTS
            .iter()
            .enumerate()
        {
            let result = blink_fn(&input, blinks);
            assert_eq!(result, expected, "Failed at blink {}", blinks);
        }
    }

    #[test]
    fn test_one_to_seven_blinks_brute() {
        run_one_to_seven_blinks(blink_brute);
    }

    #[test]
    fn test_one_to_seven_blinks_mapped() {
        run_one_to_seven_blinks(blink_mapped);
    }

    #[test]
    fn test_25_blinks_brute() {
        let result = blink_brute(&INPUT.to_vec(), 25);
        assert_eq!(result, EXPECTED_25_BLINKS);
    }

    #[test]
    fn test_25_blinks_mapped() {
        let result = blink_mapped(&INPUT.to_vec(), 25);
        assert_eq!(result, EXPECTED_25_BLINKS);
    }
}
