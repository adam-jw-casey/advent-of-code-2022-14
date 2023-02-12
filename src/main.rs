use std::{fs,env};
use advent_of_code_2022_14::resting_sand;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read file");

    println!("{}", resting_sand(&contents));
}
