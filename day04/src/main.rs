use std::fs;
use std::str;

fn main() {
    const FILEPATH: &str = "input.txt";
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let overlapped: Vec<bool> = contents
        .split("\n")
        // Filtermap maybe
        .map(|pair| match fully_contains_assignment(pair) {
            Ok(true) => true,
            Ok(false) => false,
            Err(msg) => {
                panic!("Mayday! Mayday! {msg}")
            }
        })
        .filter(|&x| x == true)
        .collect();

    println!("Length 4a: {}", overlapped.len());

    let partially_overlapped: Vec<bool> = contents
        .split("\n")
        // Filtermap maybe
        .map(|pair| match partially_contains_assignment(pair) {
            Ok(true) => true,
            Ok(false) => false,
            Err(msg) => {
                panic!("Mayday! Mayday! {msg}")
            }
        })
        .filter(|&x| x == true)
        .collect();

    println!("Length 4b) {}", partially_overlapped.len());
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
