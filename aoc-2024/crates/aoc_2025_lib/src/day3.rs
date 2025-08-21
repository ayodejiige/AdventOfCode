use regex::Regex;
use std::fs;

fn mull_it_over(file_path: &String) -> i64 {
    let content = fs::read_to_string(file_path).unwrap();
    let mut result: i64 = 0;

    let re = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let mut it = re.captures_iter(content.as_str());
    while let Some(caps) = it.next() {
        let num1 = i64::from_str_radix(&caps["num1"], 10).unwrap();
        let num2 = i64::from_str_radix(&caps["num2"], 10).unwrap();

        result += num1 * num2;
    }

    result
}

fn mull_it_over_ext(file_path: &String) -> i64 {
    let content = fs::read_to_string(file_path).unwrap();
    let mut result: i64 = 0;
    let mut skip: bool = false;

    let re =
        Regex::new(r"(mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))")
            .unwrap();
    let mut it = re.captures_iter(content.as_str());
    while let Some(caps) = it.next() {
        if let Some(_) = caps.name("do") {
            skip = false;
            continue;
        }

        if let Some(_) = caps.name("dont") {
            skip = true;
            continue;
        }

        if skip {
            continue;
        }

        let num1 = i64::from_str_radix(&caps["num1"], 10).unwrap();
        let num2 = i64::from_str_radix(&caps["num2"], 10).unwrap();

        result += num1 * num2;
    }

    result
}

pub fn main(file_path: String) {
    let ans = mull_it_over(&file_path);
    println!("Sum: {}", ans);
    let ans = mull_it_over_ext(&file_path);
    println!("Sum Corrected: {}", ans);
}
