use std::fs;
use std::str;

fn main() {
    const FILEPATH: &str = "input.txt";
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let wrong_items = contents
        .split("\n")
        .map(find_duplicate_in_rucksack)
        .collect::<Vec<char>>();

    let dup = &wrong_items[2];
    println!("Result: {dup}");
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
                println!("{first_item} {second_item}");
                return *first_item;
            }
        }
    }
    panic!("Duplicate item not found.");
}
