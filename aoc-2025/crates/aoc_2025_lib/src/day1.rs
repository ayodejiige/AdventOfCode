use crate::common::{calculate_distance, calculate_similarity, quick_sort};

use regex::Regex;
use std::fs;

fn read_input_file(file_path: &String) -> Result<(Vec<i64>, Vec<i64>), &'static str> {
    let mut list_one = Vec::<i64>::new();
    let mut list_two: Vec<i64> = Vec::<i64>::new();

    let content = fs::read_to_string(file_path).expect("File should be valid");

    for line in content.lines() {
        let re = Regex::new(r"\s+").unwrap();
        let numbers: Vec<&str> = re.split(line).collect::<Vec<&str>>();

        let num_one = i64::from_str_radix(numbers[0], 10).unwrap();
        let num_two = i64::from_str_radix(numbers[1], 10).unwrap();

        list_one.push(num_one);
        list_two.push(num_two);
    }

    Ok((list_one, list_two))
}

pub fn main(file_path: String) {
    let (mut list_one, mut list_two) = read_input_file(&file_path).unwrap();
    quick_sort(&mut list_one, |a, b| a.cmp(b));
    quick_sort(&mut list_two, |a, b| a.cmp(b));

    let distance = calculate_distance(&list_one, &list_two).unwrap();
    let similarity = calculate_similarity(&list_one, &list_two).unwrap();
    println!("Distance is {distance}");
    println!("Similarity is {similarity}");
}