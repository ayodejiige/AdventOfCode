use regex::Regex;
use std::fs;

fn is_safe_magnitude(prev: i64, curr: i64) -> bool {
    let diff = i64::abs(prev - curr);

    diff >= 1 && diff <= 3
}

fn is_safe_direction(prev: i64, curr: i64, increase_expected: bool) -> bool {
    let increasing = curr > prev;

    increasing == increase_expected
}

fn is_report_safe(reports: &Vec<i64>) -> bool {
    let mut prev_num = reports[0];
    let mut increasing = false;

    for idx in 1..reports.len() {
        if idx == 1 {
            increasing = reports[idx] > prev_num;
        }

        if !is_safe_direction(prev_num, reports[idx], increasing) {
            return false;
        }

        if !is_safe_magnitude(prev_num, reports[idx]) {
            return false;
        }

        prev_num = reports[idx]
    }

    true
}

fn is_report_safe_ext(reports: &Vec<i64>) -> bool {
    if is_report_safe(reports) {
        return true;
    }

    // If report is not safe, check if report can be made safe by
    // removing a single element.
    for idx in 0..reports.len() {
        let mut reports_dampened = reports.clone();
        reports_dampened.remove(idx);
        if is_report_safe(&reports_dampened) {
            return true;
        }
    }

    false
}

fn calculate_safety(file_path: &String) -> Result<i64, &'static str> {
    let mut safe_count: i64 = 0;
    let content: String = fs::read_to_string(file_path).unwrap();

    for line in content.lines() {
        let re = Regex::new(r"\s+").unwrap();
        let numbers: Vec<&str> = re
            .split(line)
            .collect::<Vec<&str>>();
        let numbers = numbers
            .iter()
            .map(|&x| i64::from_str_radix(x, 10).unwrap())
            .collect::<Vec<i64>>();

        if is_report_safe_ext(&numbers) == true {
            safe_count = safe_count + 1;
        }
    }

    Ok(safe_count)
}

pub fn main(file_path: String) {
    let ans = calculate_safety(&file_path).unwrap();

    println!("Safe reports: {}", ans);
}
