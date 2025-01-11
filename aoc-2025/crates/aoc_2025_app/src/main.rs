use std::env;
use std::path::Path;
use paste::paste;

macro_rules! match_days {
    ($curr_day:expr, $($day_num:tt),*) => {
        match $curr_day {
            $(
                $day_num => {
                    paste! {
                        aoc_2025_lib::[<day $day_num>]::main
                    }
                },
            )*
            _ => {
                println!("Invalid day: {}", $curr_day);
                return;
            }
        }
    };
}
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
    let _main = match_days!(day, 1, 2, 3, 4, 5, 6, 7, 8);

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