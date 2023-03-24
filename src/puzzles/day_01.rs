use crate::{PuzzleResult, SubTaskResult};

pub fn solve(input: &str) -> PuzzleResult {
    let elves = input.split("\n\n");
    let calories_per_elf: Vec<u64> = elves
        .map(|elf| elf.split("\n"))
        .map(|elf_calories| elf_calories.map(|x| x.parse::<u64>()).flatten())
        .map(|calories| calories.sum())
        .collect::<Vec<u64>>();
    let most_calories_carried = calories_per_elf.iter().max().unwrap();

    PuzzleResult {
        task_a: SubTaskResult {
            description: String::from("1a) Most calories carried"),
            result: (*most_calories_carried).to_string(),
        },
        task_b: SubTaskResult {
            description: String::from("1b) Calories carried by top 3"),
            result: calories_carried_by_top_three(&calories_per_elf).to_string(),
        },
    }
}

fn calories_carried_by_top_three(calories_per_elf: &Vec<u64>) -> u64 {
    let mut sorted_calories_per_elf = calories_per_elf.clone();
    sorted_calories_per_elf.sort_by(|a, b| b.cmp(a));

    sorted_calories_per_elf[0] + sorted_calories_per_elf[1] + sorted_calories_per_elf[2]
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_01::solve;

    #[test]
    fn it_should_solve_day01() {
        let input = "1000\n\
        2000\n\
        3000\n\n\
        4000\n\n\
        5000\n\
        6000\n\n\
        7000\n\
        8000\n\
        9000\n\n\
        10000";
        let result = solve(input);
        assert_eq!(result.task_a.result, "24000");
        assert_eq!(result.task_b.result, "45000");
    }
}
