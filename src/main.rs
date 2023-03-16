use std::{env, fs};

use rust_advent_of_code::{run, PuzzleConfig};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to add the puzzle number argument");
    }
    let puzzle_number = args[1].clone().parse::<u64>().unwrap();
    let input_file_suffix = if puzzle_number < 10 {
        format!("0{}", puzzle_number)
    } else {
        puzzle_number.to_string()
    };
    let file_path = format!("./input_texts/day_{}.txt", input_file_suffix);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let config = PuzzleConfig::new(puzzle_number, contents);
    run(config)
}
