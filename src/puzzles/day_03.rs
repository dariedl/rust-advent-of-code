use crate::{PuzzleResult, SubTaskResult};

pub fn solve(input: &str) -> PuzzleResult {
    let sum_of_duplicates = input
        .split("\n")
        .map(find_duplicate_in_rucksack)
        .map(map_to_digit)
        .sum::<u64>();

    let rucksacks = input.split("\n").collect::<Vec<&str>>();
    let sum_of_group_duplicates = rucksacks
        .chunks(3)
        .map(|chunk| find_duplicate_in_group(chunk[0], chunk[1], chunk[2]))
        .map(map_to_digit)
        .sum::<u64>();

    PuzzleResult {
        task_a: SubTaskResult {
            description: String::from("3a) Sum of duplicates"),
            result: sum_of_duplicates.to_string(),
        },
        task_b: SubTaskResult {
            description: String::from("3b) Sum of group duplicates"),
            result: sum_of_group_duplicates.to_string(),
        },
    }
}

fn find_duplicate_in_rucksack(rucksack: &str) -> char {
    let compartment_length = rucksack.len() / 2;
    let compartment1 = &rucksack[..compartment_length];
    let compartment2 = &rucksack[compartment_length..];

    compartment1
        .chars()
        .find(|first_item| {
            compartment2
                .chars()
                .any(|second_item| &second_item == first_item)
        })
        .unwrap()
}

fn map_to_digit(letter: char) -> u64 {
    if letter.is_lowercase() {
        letter as u64 - 96
    } else if letter.is_uppercase() {
        letter as u64 - 38
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

#[cfg(test)]
mod tests {
    use crate::puzzles::day_03::solve;

    #[test]
    fn it_should_solve_day03() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = solve(input);
        assert_eq!(result.task_a.result, "157");
        assert_eq!(result.task_b.result, "70");
    }
}
