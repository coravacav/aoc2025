//! --- Day 8: Playground ---
//!
//! Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.
//!
//! You rematerialize on an unfamiliar teleporter pad and find yourself in a vast underground space which contains a giant playground!
//!
//! Across the playground, a group of Elves are working on setting up an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes.
//!
//! Their plan is to connect the junction boxes with long strings of lights. Most of the junction boxes don't provide electricity; however, when two junction boxes are connected by a string of lights, electricity can pass between those two junction boxes.
//!
//! The Elves are trying to figure out which junction boxes to connect so that electricity can reach every junction box. They even have a list of all of the junction boxes' positions in 3D space (your puzzle input).
//!
//! For example:
//!
//! ```text
//! 162,817,812
//! 57,618,57
//! 906,360,560
//! 592,479,940
//! 352,342,300
//! 466,668,158
//! 542,29,236
//! 431,825,988
//! 739,650,466
//! 52,470,668
//! 216,146,977
//! 819,987,18
//! 117,168,530
//! 805,96,715
//! 346,949,466
//! 970,615,88
//! 941,993,340
//! 862,61,35
//! 984,92,344
//! 425,690,689
//! ```
//! This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.
//!
//! To save on string lights, the Elves would like to focus on connecting pairs of junction boxes that are as close together as possible according to straight-line distance. In this example, the two junction boxes which are closest together are 162,817,812 and 425,690,689.
//!
//! By connecting these two junction boxes together, because electricity can flow between them, they become part of the same circuit. After connecting them, there is a single circuit which contains two junction boxes, and the remaining 18 junction boxes remain in their own individual circuits.
//!
//! Now, the two junction boxes which are closest together but aren't already directly connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812 is already connected to another junction box, there is now a single circuit which contains three junction boxes and an additional 17 circuits which contain one junction box each.
//!
//! The next two junction boxes to connect are 906,360,560 and 805,96,715. After connecting them, there is a circuit containing 3 junction boxes, a circuit containing 2 junction boxes, and 15 circuits which contain one junction box each.
//!
//! The next two junction boxes are 431,825,988 and 425,690,689. Because these two junction boxes were already in the same circuit, nothing happens!
//!
//! This process continues for a while, and the Elves are concerned that they don't have enough extension cables for all these circuits. They would like to know how big the circuits will be.
//!
//! After making the ten shortest connections, there are 11 circuits: one circuit which contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits which contain 2 junction boxes each, and seven circuits which each contain a single junction box. Multiplying together the sizes of the three largest circuits (5, 4, and one of the circuits of size 2) produces 40.
//!
//! Your list contains many junction boxes; connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits?

use std::collections::HashMap;

use itertools::Itertools;

use crate::Solution;

pub struct Day8 {}

impl Solution for Day8 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        let coords: Vec<(i64, i64, i64)> = input
            .lines()
            .map(|line| {
                let parts: Vec<i64> = line
                    .split(',')
                    .map(|part| part.parse::<i64>().unwrap())
                    .collect();
                (parts[0], parts[1], parts[2])
            })
            .collect();

        let mut junctions: HashMap<(i64, i64, i64), usize> = HashMap::new();

        let mut distances = coords
            .iter()
            .combinations(2)
            .map(|v| {
                let a = *v[0];
                let b = *v[1];
                let dist = ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64;
                (dist.sqrt(), a, b)
            })
            .collect::<Vec<(f64, (i64, i64, i64), (i64, i64, i64))>>();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (_, a, b) in distances.iter().take(if cfg!(test) { 10 } else { 1000 }) {
            let group_a = junctions.get(a).cloned();
            let group_b = junctions.get(b).cloned();

            match (group_a, group_b) {
                (Some(ga), Some(gb)) if ga != gb => {
                    let min_group = ga.min(gb);
                    let max_group = ga.max(gb);
                    for (_, group) in junctions.iter_mut() {
                        if *group == max_group {
                            *group = min_group;
                        }
                    }
                }
                (Some(ga), None) => {
                    junctions.insert(*b, ga);
                }
                (None, Some(gb)) => {
                    junctions.insert(*a, gb);
                }
                (None, None) => {
                    let new_group = junctions.len();
                    junctions.insert(*a, new_group);
                    junctions.insert(*b, new_group);
                }
                _ => {}
            }
        }

        let mut group_sizes: HashMap<usize, usize> = HashMap::new();
        for group in junctions.values() {
            *group_sizes.entry(*group).or_insert(0) += 1;
        }
        let mut sizes: Vec<usize> = group_sizes.values().cloned().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        (sizes[0] * sizes[1] * sizes[2]).to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let coords: Vec<(i64, i64, i64)> = input
            .lines()
            .map(|line| {
                let parts: Vec<i64> = line
                    .split(',')
                    .map(|part| part.parse::<i64>().unwrap())
                    .collect();
                (parts[0], parts[1], parts[2])
            })
            .collect();

        let mut junctions: HashMap<(i64, i64, i64), usize> = HashMap::new();

        let mut distances = coords
            .iter()
            .combinations(2)
            .map(|v| {
                let a = *v[0];
                let b = *v[1];
                let dist = ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64;
                (dist.sqrt(), a, b)
            })
            .collect::<Vec<(f64, (i64, i64, i64), (i64, i64, i64))>>();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (_, a, b) in distances.iter() {
            let group_a = junctions.get(a).cloned();
            let group_b = junctions.get(b).cloned();

            match (group_a, group_b) {
                (Some(ga), Some(gb)) if ga != gb => {
                    let min_group = ga.min(gb);
                    let max_group = ga.max(gb);
                    for (_, group) in junctions.iter_mut() {
                        if *group == max_group {
                            *group = min_group;
                        }
                    }

                    if min_group == 0 && junctions.len() == coords.len() {
                        return (a.0 * b.0).to_string();
                    }
                }
                (Some(ga), None) => {
                    junctions.insert(*b, ga);
                }
                (None, Some(gb)) => {
                    junctions.insert(*a, gb);
                }
                (None, None) => {
                    let new_group = junctions.len();
                    junctions.insert(*a, new_group);
                    junctions.insert(*b, new_group);
                }
                _ => {}
            }

            if junctions.len() == coords.len() && junctions.values().all(|&g| g == 0) {
                return (a.0 * b.0).to_string();
            }
        }

        dbg!(junctions.len(), coords.len());

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day8::new();
        assert_eq!(
            solution.part1(
                r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
            ),
            String::from("40")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day8::new();
        assert_eq!(
            solution.part2(
                r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
            ),
            String::from("25272")
        );
    }
}
