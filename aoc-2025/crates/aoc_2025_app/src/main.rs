use std::env;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    
    if args.len() < 3 {
        println!("Error: Invalid arguments");
        return;
    }

    let day_arg = &args[1];
    let input_folder_arg = &args[2];

    // Select main function to run.
    let day = u32::from_str_radix(day_arg, 10).unwrap();
    let _main = match day {
        1 => {
            aoc_2025_lib::day1::main
        },
        2 => {
            aoc_2025_lib::day2::main
        }
        3 => {
            aoc_2025_lib::day3::main
        }
        4 => {
            aoc_2025_lib::day4::main
        }
        _ => {
            println!("Invalid day: {day}");
            return;
        }
    };

    // Check that input folder exists.
    if !Path::new(input_folder_arg).exists() {
        println!("Invalid input path: {input_folder_arg}");
        return;
    }

    let input_file = format!("{input_folder_arg}/day{day_arg}.txt");
    // Check that input file exists.
    if !Path::new(&input_file).exists() {
        println!("Invalid input path: {input_folder_arg}");
        return;
    }

    // Run main function.
    _main(input_file);
}