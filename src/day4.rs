//! --- Day 4: Printing Department ---
//!
//! You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).
//!
//! Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.
//!
//! "Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."
//!
//! If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.
//!
//! The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.
//!
//! For example:
//!
//! ```text
//! ..@@.@@@@.
//! @@@.@.@.@@
//! @@@@@.@.@@
//! @.@@@@..@.
//! @@.@@@@.@@
//! .@@@@@@@.@
//! .@.@.@.@@@
//! @.@@@.@@@@
//! .@@@@@@@@.
//! @.@.@@@.@.
//! ```
//! The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.
//!
//! In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):
//!
//! ```text
//! ..xx.xx@x.
//! x@@.@.@.@@
//! @@@@@.x.@@
//! @.@@@@..@.
//! x@.@@@@.@x
//! .@@@@@@@.@
//! .@.@.@.@@@
//! x.@@@.@@@@
//! .@@@@@@@@.
//! x.x.@@@.x.
//! ```
//! Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

use std::fmt::Display;

use itertools::Itertools;

use crate::{Solution, grid::Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Dot,
    Roll,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Dot => write!(f, "."),
            Type::Roll => write!(f, "@"),
        }
    }
}

pub struct Day4 {}

impl Solution for Day4 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let grid = Grid::new(
            input,
            |c| match c {
                '.' => Type::Dot,
                '@' => Type::Roll,
                _ => panic!("Invalid character"),
            },
            false,
        );

        grid.iter_with_coords()
            .filter(|(v, _)| **v == Type::Roll)
            .map(|(_, c)| (c, grid.survey_octo(c)))
            .filter(|(_, survey)| {
                survey
                    .iter()
                    .flatten()
                    .filter(|&&&around| around == Type::Roll)
                    .count()
                    < 4
            })
            .count()
            .to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut grid = Grid::new(
            input,
            |c| match c {
                '.' => Type::Dot,
                '@' => Type::Roll,
                _ => panic!("Invalid character"),
            },
            false,
        );

        let mut count = 0;

        loop {
            let removable = grid
                .iter_with_coords()
                .filter(|(v, _)| **v == Type::Roll)
                .map(|(_, c)| (c, grid.survey_octo(c)))
                .filter(|(_, survey)| {
                    survey
                        .iter()
                        .flatten()
                        .filter(|&&&around| around == Type::Roll)
                        .count()
                        < 4
                })
                .map(|(c, _)| c)
                .collect_vec();

            if removable.is_empty() {
                break;
            }

            for coord in &removable {
                grid.set(*coord, Type::Dot);
            }

            count += removable.len();
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day4::new();
        assert_eq!(
            solution.part1(
                r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            ),
            String::from("13")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day4::new();
        assert_eq!(
            solution.part2(
                r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            ),
            String::from("43")
        );
    }
}
