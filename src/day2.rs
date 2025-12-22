//! --- Day 2: Gift Shop ---
//!
//! You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.
//!
//! As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.
//!
//! As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?
//!
//! They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//!
//! ```text
//! 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//! 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//! 824824821-824824827,2121212118-2121212124
//! ```
//! (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//!
//! The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
//!
//! Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
//!
//! None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)
//!
//! Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//!
//! - `11-22` has two invalid IDs, `11` and `22`.
//! - `95-115` has one invalid ID, `99`.
//! - `998-1012` has one invalid ID, `1010`.
//! - `1188511880-1188511890` has one invalid ID, `1188511885`.
//! - `222220-222224` has one invalid ID, `222222`.
//! - `1698522-1698528` contains no invalid IDs.
//! - `446443-446449` has one invalid ID, `446446`.
//! - `38593856-38593862` has one invalid ID, `38593859`.
//! - The rest of the ranges contain no invalid IDs.
//! - Adding up all the invalid IDs in this example produces `1227775554`.
//!
//! What do you get if you add up all of the invalid IDs?

use crate::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut total: u64 = 0;

        for range in input.trim().split(",") {
            let (lower, upper) = range.split_once('-').unwrap();
            let lower: u64 = lower.parse().unwrap();
            let upper: u64 = upper.parse().unwrap();
            if lower.ilog10() % 2 == 0 && upper.ilog10() % 2 == 0 {
                continue;
            }

            for x in lower..upper + 1 {
                let xstr = x.to_string();
                if xstr[..xstr.len() / 2] == xstr[xstr.len() / 2..] {
                    total += x;
                }
            }
        }

        total.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut total: u64 = 0;

        for range in input.trim().split(",") {
            let (lower, upper) = range.split_once('-').unwrap();
            let lower: u64 = lower.parse().unwrap();
            let upper: u64 = upper.parse().unwrap();

            'next_number: for x in lower..=upper {
                let xstr = x.to_string();
                'digit_count: for i in 1..=xstr.len() / 2 {
                    if xstr.len() % i != 0 {
                        continue 'digit_count;
                    }

                    for piece in 0..xstr.len() / i {
                        if xstr[i * piece..i * (piece + 1)] != xstr[..i] {
                            continue 'digit_count;
                        }
                    }

                    total += x;
                    continue 'next_number;
                }
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day2::new();
        assert_eq!(
            solution.part1(
                r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#
            ),
            String::from("1227775554")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day2::new();
        assert_eq!(
            solution.part2(
                r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#
            ),
            String::from("4174379265")
        );
    }
}
