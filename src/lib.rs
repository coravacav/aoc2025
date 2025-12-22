pub trait Solution {
    fn new() -> Self
    where
        Self: Sized;

    fn part1(&mut self, _input: &str) -> String {
        "".to_string()
    }
    fn part2(&mut self, _input: &str) -> String {
        "".to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        None
    }

    fn known_solution_part2(&self) -> Option<String> {
        None
    }
}

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod direction;
pub mod grid;

pub fn get_solution(day: u8) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1::new()),
        2 => Box::new(day2::Day2::new()),
        3 => Box::new(day3::Day3::new()),
        4 => Box::new(day4::Day4::new()),
        5 => Box::new(day5::Day5::new()),
        6 => Box::new(day6::Day6::new()),
        7 => Box::new(day7::Day7::new()),
        8 => Box::new(day8::Day8::new()),
        9 => Box::new(day9::Day9::new()),
        10 => Box::new(day10::Day10::new()),
        11 => Box::new(day11::Day11::new()),
        12 => Box::new(day12::Day12::new()),
        _ => panic!("Invalid day"),
    }
}

pub fn get_input(day: u8) -> String {
    // File path is in inputs/{}_input.txt
    std::fs::read_to_string(format!("inputs/{}_input.txt", day)).unwrap()
}

pub fn panic_after(count: usize) {
    static mut COUNT: usize = 0;
    unsafe {
        COUNT += 1;
        if COUNT >= count {
            panic!("Panic after {} iterations", count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_all_parts() {
        for day in 1..=25 {
            let mut solution = get_solution(day);
            let input = get_input(day);
            let input = input.trim();

            let part1 = solution.part1(input);
            let part2 = solution.part2(input);

            if let Some(known_solution_part1) = solution.known_solution_part1() {
                assert_eq!(
                    known_solution_part1, part1,
                    "Part 1 of day {} is wrong",
                    day
                );
            }

            if let Some(known_solution_part2) = solution.known_solution_part2() {
                assert_eq!(
                    known_solution_part2, part2,
                    "Part 2 of day {} is wrong",
                    day
                );
            }
        }
    }
}
