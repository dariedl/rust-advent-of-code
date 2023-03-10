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
    println!("{sum_of_duplicates}");

    let rucksacks = contents.split("\n").collect::<Vec<&str>>();
    let sum_of_group_duplicates = rucksacks
        .chunks(3)
        .map(|chunk| find_duplicate_in_group(chunk[0], chunk[1], chunk[2]))
        .map(map_to_digit)
        .sum::<i64>();

    println!("{sum_of_group_duplicates}");
}

fn find_duplicate_in_rucksack(rucksack: &str) -> char {
    let compartment_length = rucksack.len() / 2;

    // Why can i not initialize it earlier? let compartment2 = rucksack[compartment_length..].chars();
    rucksack[..compartment_length]
        .chars()
        .filter(|first_item| {
            rucksack[compartment_length..]
                .chars()
                .any(|second_item| &second_item == first_item)
        })
        .next()
        .unwrap()
}

fn map_to_digit(letter: char) -> i64 {
    if letter.is_lowercase() {
        letter as i64 - 96
    } else if letter.is_uppercase() {
        letter as i64 - 38
    } else {
        panic!("Not a letter");
    }
}

fn find_duplicate_in_group(a_rucksack: &str, b_rucksack: &str, c_rucksack: &str) -> char {
    a_rucksack
        .chars()
        .filter(|a| b_rucksack.chars().any(|b| &b == a) && c_rucksack.chars().any(|c| &c == a))
        .next()
        .unwrap()
}
