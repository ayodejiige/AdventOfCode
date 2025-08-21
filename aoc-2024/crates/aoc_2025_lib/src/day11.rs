use crate::common::number_of_digits;
use std::{collections::HashMap, fs, time::Instant};

enum SingleOrSplit {
    Single(u64),
    Split(u64, u64),
}

fn transform_stone(stone: u64) -> SingleOrSplit {
    let digit_count = number_of_digits(stone) as u32;
    if stone == 0 {
        SingleOrSplit::Single(1)
    } else if digit_count % 2 == 0 {
        let left_digit = stone / 10_u64.pow(digit_count / 2);
        let right_digit = stone - (left_digit * 10_u64.pow(digit_count / 2));
        SingleOrSplit::Split(left_digit, right_digit)
    } else {
        SingleOrSplit::Single(stone * 2024)
    }
}

fn blink_brute(stones: &Vec<u64>, blinks: usize) -> usize {
    let mut stones = stones.clone();
    for _ in 0..blinks {
        let mut new_stones = Vec::<u64>::new();

        for stone in stones {
            match transform_stone(stone) {
                SingleOrSplit::Single(new_stone) => {
                    new_stones.push(new_stone);
                }
                SingleOrSplit::Split(new_stone_1, new_stone_2) => {
                    new_stones.push(new_stone_1);
                    new_stones.push(new_stone_2);
                }
            }
        }

        stones = new_stones;
    }

    stones.len()
}

fn blink_mapped(stones: &Vec<u64>, blinks: usize) -> usize {
    let mut stone_map: HashMap<u64, usize> = HashMap::new();

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
                SingleOrSplit::Single(new_stone) => {
                    new_stone_map
                        .entry(new_stone)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
                SingleOrSplit::Split(stone_1, stone_2) => {
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

#[allow(unused)]
fn profile_blinks() {
    let stones = vec![125, 17];
    for blinks in 0..40 {
        let brute_start = Instant::now();
        let brute_ans = blink_brute(&stones, blinks);
        let brute_elapsed = brute_start
            .elapsed()
            .as_micros();

        let mapped_start = Instant::now();
        let mapped_ans = blink_mapped(&stones, blinks);
        let mapped_elapsed = mapped_start
            .elapsed()
            .as_micros();

        println!(
            "{} blinks: Brute =>({} stones ,{}us) Mapped => ({} stones ,{}us)",
            blinks, brute_ans, brute_elapsed, mapped_ans, mapped_elapsed
        );
    }
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
