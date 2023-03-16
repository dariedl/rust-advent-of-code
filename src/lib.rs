mod puzzles;

pub struct PuzzleConfig {
    number: u64,
    input: String,
}

impl PuzzleConfig {
    pub fn new(number: u64, input: String) -> PuzzleConfig {
        PuzzleConfig { number, input }
    }
}

pub fn run(config: PuzzleConfig) {
    match config.number {
        1 => puzzles::day_01::solve(config.input),
        2 => puzzles::day_02::solve(config.input),
        3 => puzzles::day_03::solve(config.input),
        4 => puzzles::day_04::solve(config.input),
        _ => panic!("We don't have this puzzle"),
    }
}