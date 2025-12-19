pub trait Solution {
    fn new() -> Self
    where
        Self: Sized;

    fn part1(&mut self, input: &str) -> String {
        input.to_string()
    }
    fn part2(&mut self, input: &str) -> String {
        input.to_string()
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
        3 => Box::new(day3::Day3::new()),
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
