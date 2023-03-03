use std::fs;
use std::str;

fn main() {
    const FILEPATH: &str = "input.txt";
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let sum_of_duplicates = contents
        .split("\n")
        .map(find_duplicate_in_rucksack)
        .map(map_to_digit)
        .sum::<i64>();
    println!("{sum_of_duplicates}")
}

fn find_duplicate_in_rucksack(rucksack: &str) -> char {
    let compartment_length = rucksack.len() / 2;

    let compartment1 = rucksack[..compartment_length]
        .chars()
        .collect::<Vec<char>>();
    let compartment2 = rucksack[compartment_length..]
        .chars()
        .collect::<Vec<char>>();

    for first_item in &compartment1 {
        for second_item in &compartment2 {
            if first_item == second_item {
                return *first_item;
            }
        }
    }
    panic!("Duplicate item not found.");
}

fn map_to_digit(letter: char) -> i64 {
    if letter.is_lowercase() {
        return letter as i64 - 96;
    } else if letter.is_uppercase() {
        return letter as i64 - 38;
    } else {
        panic!("Not a letter");
    }
}
