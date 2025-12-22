//! --- Day 5: Cafeteria ---
//!
//! As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.
//!
//! You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.
//!
//! "If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.
//!
//! The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).
//!
//! The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:
//!
//! ```text
//! 3-5
//! 10-14
//! 16-20
//! 12-18
//!
//! 1
//! 5
//! 8
//! 11
//! 17
//! 32
//! ```
//! The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.
//!
//! The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:
//!
//! - Ingredient ID 1 is spoiled because it does not fall into any range.
//! - Ingredient ID 5 is fresh because it falls into range 3-5.
//! - Ingredient ID 8 is spoiled.
//! - Ingredient ID 11 is fresh because it falls into range 10-14.
//! - Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
//! - Ingredient ID 32 is spoiled.
//!
//! So, in this example, 3 of the available ingredient IDs are fresh.
//!
//! Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

use crate::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut ranges = vec![];

        let mut lines = input.lines();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (lower, upper) = line.split_once('-').unwrap();
            let lower: u64 = lower.parse().unwrap();
            let upper: u64 = upper.parse().unwrap();
            ranges.push(lower..=upper);
        }

        let mut total = 0;

        for line in lines.by_ref() {
            let id: u64 = line.parse().unwrap();
            if ranges.iter().any(|range| range.contains(&id)) {
                total += 1;
            }
        }

        total.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut ranges = vec![];

        for line in input.lines() {
            if line.is_empty() {
                break;
            }
            let (lower, upper) = line.split_once('-').unwrap();
            let lower: u64 = lower.parse().unwrap();
            let upper: u64 = upper.parse().unwrap();
            ranges.push((lower, upper));
        }

        // Go through each range and remove overlapping ranges
        ranges.sort_by_key(|range| range.0);
        let mut merged_ranges = vec![];
        let mut current_range = ranges[0].clone();

        for range in ranges.iter().skip(1) {
            if range.0 <= current_range.1 + 1 {
                current_range.1 = current_range.1.max(range.1);
            } else {
                merged_ranges.push(current_range);
                current_range = range.clone();
            }
        }

        merged_ranges.push(current_range);

        let mut total = 0;
        for range in merged_ranges {
            total += range.1 - range.0 + 1;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day5::new();
        assert_eq!(
            solution.part1(
                r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
            ),
            String::from("3")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day5::new();
        assert_eq!(
            solution.part2(
                r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
            ),
            String::from("14")
        );
    }
}
