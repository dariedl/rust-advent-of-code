pub fn solve(input: String) {
    let elves = input.split("\n\n");
    let calories_per_elf: Vec<u64> = elves
        .map(|elf| elf.split("\n"))
        .map(|elf_calories| {
            elf_calories.map(|x| x.parse::<u64>()).flatten()
            // .collect::<Result<Vec<u64>, ParseIntError>>()
        })
        .map(|calories| calories.sum())
        .collect::<Vec<u64>>();
    most_calories_carried(&calories_per_elf);
    calories_carried_by_top_three(&calories_per_elf);
}

fn most_calories_carried(calories_per_elf: &Vec<u64>) {
    let max_value = calories_per_elf.iter().max();
    match max_value {
        Some(max) => println!("Most calories carried: {}", max),
        None => println!("Vector is empty"),
    }
}

fn calories_carried_by_top_three(calories_per_elf: &Vec<u64>) {
    let mut sorted_calories_per_elf = calories_per_elf.clone();
    sorted_calories_per_elf.sort_by(|a, b| b.cmp(a));

    let calories_carried_by_top_three =
        sorted_calories_per_elf[0] + sorted_calories_per_elf[1] + sorted_calories_per_elf[2];
    println!("Calories carried by top 3: {calories_carried_by_top_three}")
}
