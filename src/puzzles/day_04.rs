use crate::{PuzzleResult, SubTaskResult};

pub fn solve(input: &str) -> PuzzleResult {
    let overlapped: Vec<bool> = input
        .split("\n")
        .filter_map(|pair| match fully_contains_assignment(pair) {
            Ok(true) => Some(true),
            Ok(false) => None,
            Err(msg) => {
                panic!("Mayday! Mayday! {msg}")
            }
        })
        .collect();

    let partially_overlapped: Vec<bool> = input
        .split("\n")
        .map(|pair| match partially_contains_assignment(pair) {
            Ok(true) => true,
            Ok(false) => false,
            Err(msg) => {
                panic!("Mayday! Mayday! {msg}")
            }
        })
        .filter(|&x| x == true)
        .collect();

    PuzzleResult {
        task_a: SubTaskResult {
            description: String::from("4a) No. of complete overlaps"),
            result: overlapped.len() as u64,
        },
        task_b: SubTaskResult {
            description: String::from("4b) No. of partial overlaps"),
            result: partially_overlapped.len() as u64,
        },
    }
}

fn fully_contains_assignment(pair: &str) -> Result<bool, String> {
    match pair.split_once(",") {
        None => Err(format!("Could not split pair: {pair}")),
        Some((first, second)) => {
            do_sections_completly_overlap(section_range_of(first), section_range_of(second))
        }
    }
}

fn partially_contains_assignment(pair: &str) -> Result<bool, String> {
    match pair.split_once(",") {
        None => Err(format!("Could not split pair: {pair}")),
        Some((first, second)) => {
            do_sections_partially_overlap(section_range_of(first), section_range_of(second))
        }
    }
}

struct SectionRange {
    from: u64,
    to: u64,
}

impl SectionRange {
    fn overlaps_completely(&self, other: SectionRange) -> bool {
        (&self.from <= &other.from && &self.to >= &other.to)
            || (&self.from >= &other.from && &self.to <= &other.to)
    }

    fn overlaps_partially(&self, other: SectionRange) -> bool {
        !((&self.to < &other.from) || (&other.to < &self.from))
    }
}

fn do_sections_completly_overlap(
    section_a_result: Result<SectionRange, String>,
    section_b_result: Result<SectionRange, String>,
) -> Result<bool, String> {
    section_a_result.and_then(|section_a| {
        section_b_result.and_then(|section_b| Ok(section_a.overlaps_completely(section_b)))
    })
}

fn do_sections_partially_overlap(
    section_a_result: Result<SectionRange, String>,
    section_b_result: Result<SectionRange, String>,
) -> Result<bool, String> {
    section_a_result.and_then(|section_a| {
        section_b_result.and_then(|section_b| Ok(section_a.overlaps_partially(section_b)))
    })
}

fn section_range_of(range: &str) -> Result<SectionRange, String> {
    match range.split_once("-") {
        None => Err(format!("Could not split range: {range}")),
        Some((from, to)) => match (from.parse::<u64>(), to.parse::<u64>()) {
            (Ok(f), Ok(t)) => Ok(SectionRange { from: f, to: t }),
            _ => Err(format!("Could not parse range: {range}")),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day_04::solve;

    #[test]
    fn it_should_solve_day04() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        let result = solve(input);
        assert_eq!(result.task_a.result, 2);
        assert_eq!(result.task_b.result, 4);
    }
}
