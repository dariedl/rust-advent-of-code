use crate::{PuzzleResult, SubTaskResult};

pub fn solve(input: &str) -> PuzzleResult {
    let rounds = Vec::from_iter(input.split("\n"));
    PuzzleResult {
        task_a: SubTaskResult {
            description: String::from("2a) Final score"),
            result: rounds
                .iter()
                .map(|round| round_score_a(round))
                .sum::<u64>()
                .to_string(),
        },
        task_b: SubTaskResult {
            description: String::from("2b) Final score"),
            result: rounds
                .iter()
                .map(|round| round_score_b(round))
                .sum::<u64>()
                .to_string(),
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

// A ------

fn map_to_shape_a(text: &str) -> Shape {
    match text {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissor,
        _ => panic!("WAAAH"),
    }
}

fn calc_score_a(opponent_move: Shape, own_move: Shape) -> u64 {
    let shape_score = match own_move {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };
    let outcome_score = match (opponent_move, own_move) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Rock, Shape::Scissor) => 0,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Scissor) => 6,
        (Shape::Scissor, Shape::Rock) => 6,
        (Shape::Scissor, Shape::Paper) => 0,
        (Shape::Scissor, Shape::Scissor) => 3,
    };
    return shape_score + outcome_score;
}

fn round_score_a(round: &str) -> u64 {
    let round_moves = Vec::from_iter(round.split(" "));
    let (opponent_move, own_move) = (
        map_to_shape_a(round_moves[0]),
        map_to_shape_a(round_moves[1]),
    );
    return calc_score_a(opponent_move, own_move);
}

// B ------

fn map_to_outcome_b(text: &str) -> u64 {
    match text {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("WAAAH"),
    }
}

fn calc_score_b(opponent_move: Shape, outcome: u64) -> u64 {
    let own_move = match (opponent_move, outcome) {
        (Shape::Rock, 3) => Shape::Rock,
        (Shape::Rock, 6) => Shape::Paper,
        (Shape::Rock, 0) => Shape::Scissor,
        (Shape::Paper, 0) => Shape::Rock,
        (Shape::Paper, 3) => Shape::Paper,
        (Shape::Paper, 6) => Shape::Scissor,
        (Shape::Scissor, 6) => Shape::Rock,
        (Shape::Scissor, 0) => Shape::Paper,
        (Shape::Scissor, 3) => Shape::Scissor,
        _ => panic!("WAAAH! B"),
    };

    let shape_score = match own_move {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };
    return shape_score + outcome;
}

fn round_score_b(round: &str) -> u64 {
    let round_moves = Vec::from_iter(round.split(" "));
    let (opponent_move, outcome) = (
        map_to_shape_a(round_moves[0]),
        map_to_outcome_b(round_moves[1]),
    );
    return calc_score_b(opponent_move, outcome);
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_02::solve;

    #[test]
    fn it_should_solve_day02() {
        let input = "A Y\n\
            B X\n\
            C Z";
        let result = solve(input);
        assert_eq!(result.task_a.result, "15");
        assert_eq!(result.task_b.result, "12");
    }
}
