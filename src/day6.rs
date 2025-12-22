//! --- Day 6: Trash Compactor ---
//!
//! After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!
//!
//! A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.
//!
//! As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her math homework.
//!
//! Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of problems; each problem has a group of numbers that need to be either added (+) or multiplied (*) together.
//!
//! However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:
//!
//! ```text
//! 123 328  51 64
//! 45 64  387 23
//! 6 98  215 314
//! *   +   *   +
//! ```
//! Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.
//!
//! So, this worksheet contains four problems:
//!
//! - `123` * `45` * `6` = `33210`
//! - `328` + `64` + `98` = `490`
//! - `51` * `387` * `215` = `4243455`
//! - `64` + `23` + `314` = `401`
//!
//! To check their work, cephalopod students are given the grand total of adding together all of the answers to the individual problems. In this worksheet, the grand total is `33210` + `490` + `4243455` + `401` = `4277556`.
//!
//! Of course, the actual worksheet is much wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.
//!
//! Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?

use crate::{Solution, grid::Grid};

pub struct Day6 {}

impl Solution for Day6 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let mut vv = vec![];
        let mut ops = vec![];

        for line in input.lines() {
            let mut v = vec![];
            for num in line.split_whitespace() {
                if let Ok(num) = num.parse::<i64>() {
                    v.push(num);
                } else {
                    ops.push(num);
                }
            }

            if !v.is_empty() {
                vv.push(v);
            }
        }

        let mut total = 0;
        for (i, op) in ops.iter().enumerate() {
            total += vv.iter().map(|v| v.iter().nth(i)).fold(
                if *op == "*" { 1 } else { 0 },
                |acc, x| {
                    if *op == "+" {
                        acc + x.unwrap()
                    } else {
                        acc * x.unwrap()
                    }
                },
            );
        }

        total.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let grid = Grid::new(input, |c| c, false);

        let mut vv = vec![];
        let mut ops = vec![];
        let mut v: Vec<i64> = vec![];

        for col in grid.iter_columns_down() {
            let mut s = String::new();

            for &v in &col {
                if v.is_numeric() {
                    s.push(*v);
                } else if *v == '*' || *v == '+' {
                    ops.push(*v);
                }
            }

            if s.is_empty() {
                if !v.is_empty() {
                    vv.push(v);
                }
                v = vec![];
            } else {
                v.push(s.parse().unwrap());
            }
        }

        vv.push(v);

        assert_eq!(vv.len(), ops.len());

        let mut total = 0;
        for (op, v) in ops.iter().zip(vv.iter()) {
            total += v.iter().fold(if *op == '*' { 1 } else { 0 }, |acc, x| {
                if *op == '+' { acc + x } else { acc * x }
            });
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_part1() {
        let mut solution = Day6::new();
        assert_eq!(
            solution.part1(
                &[
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .iter()
                .join("\n")
            ),
            String::from("4277556")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day6::new();
        assert_eq!(
            solution.part2(
                &[
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .iter()
                .join("\n")
            ),
            String::from("3263827")
        );
    }
}
