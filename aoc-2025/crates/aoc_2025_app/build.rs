use std::env;
use std::fs;
use std::path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let input_dir = manifest_dir + "/../../inputs";

    let input_path = path::Path::new(&input_dir);

    let input_files = fs::read_dir(input_path).unwrap().collect::<Vec<_>>();
    for input_file_result in input_files {
        let input_file_path = input_file_result.unwrap().path();
        let input_file_name = input_file_path.file_name().unwrap().to_str().unwrap();
        let input_file = String::from(input_file_path.as_os_str().to_str().unwrap());
        let output_file = out_dir.clone() + "/" + input_file_name;

        print!("Copying from {} to {}", input_file, output_file);
        fs::copy(input_file, output_file).expect("Should be successful");
    }
}
