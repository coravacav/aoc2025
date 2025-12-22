//! --- Day 9: Movie Theater ---
//!
//! You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!
//!
//! The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).
//!
//! For example:
//!
//! ```text
//! 7,1
//! 11,1
//! 11,7
//! 9,7
//! 9,5
//! 2,5
//! 2,3
//! 7,3
//! ```
//! Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:
//!
//! ```text
//! ..............
//! .......#...#..
//! ..............
//! ..#....#......
//! ..............
//! ..#......#....
//! ..............
//! .........#.#..
//! ..............
//! ```
//! You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.
//!
//! For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:
//!
//! ```text
//! ..............
//! .......#...#..
//! ..............
//! ..#....#......
//! ..............
//! ..OOOOOOOO....
//! ..OOOOOOOO....
//! ..OOOOOOOO.#..
//! ..............
//! ```
//! Or, you could make a rectangle with area 35 between 7,1 and 11,7:
//!
//! ```text
//! ..............
//! .......OOOOO..
//! .......OOOOO..
//! ..#....OOOOO..
//! .......OOOOO..
//! ..#....OOOOO..
//! .......OOOOO..
//! .......OOOOO..
//! ..............
//! ```
//! You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:
//!
//! ```text
//! ..............
//! .......#...#..
//! ..............
//! ..OOOOOO......
//! ..............
//! ..#......#....
//! ..............
//! .........#.#..
//! ..............
//! ```
//! Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:
//!
//! ```text
//! ..............
//! ..OOOOOOOOOO..
//! ..OOOOOOOOOO..
//! ..OOOOOOOOOO..
//! ..OOOOOOOOOO..
//! ..OOOOOOOOOO..
//! ..............
//! .........#.#..
//! ..............
//! ```
//! Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?

use dashmap::DashMap;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    Solution,
    grid::{Coord, Grid},
};

pub struct Day9 {}

impl Solution for Day9 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            })
            .combinations(2)
            .map(|pair| {
                let (x1, y1) = pair[0];
                let (x2, y2) = pair[1];
                let width = (x2 - x1).abs() + 1;
                let height = (y2 - y1).abs() + 1;
                width * height
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let coords = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Coord::new(y.parse().unwrap(), x.parse().unwrap())
            })
            .collect_vec();

        let max_row = coords.iter().max_by_key(|c| c.row()).unwrap().row() + 1;
        let max_col = coords.iter().max_by_key(|c| c.col()).unwrap().col() + 1;

        let mut grid = Grid::new_blank(max_col, max_row, false, false);

        grid.trace_coord_list(&coords, |_, _, _| true, true);

        grid.pretty_print_low_resolution_square_with_printer(
            10,
            &mut std::io::stdout().lock(),
            |vals| {
                vals.iter()
                    .map(|(v, _)| v)
                    .any(|&&v| v)
                    .then(|| "#".to_string())
                    .unwrap_or(".".to_string())
            },
        );

        let mut areas = coords
            .iter()
            .combinations(2)
            .map(|pair| (*pair[0], *pair[1]))
            .map(|(c1, c2)| (c1, c2, c1.area(c2)))
            .collect_vec();

        areas.sort_by(|a, b| b.2.cmp(&a.2));

        let cache: DashMap<Coord, bool> = DashMap::new();
        // let cache_entry: AtomicUsize = AtomicUsize::new(0);

        let max = areas
            .par_iter()
            .filter(|(c1, c2, _)| {
                c1.other_corners(*c2).iter().all(|c| {
                    *grid.get(*c).unwrap()
                        || *cache
                            .entry(*c)
                            // .and_modify(|_| {
                            //     cache_entry.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            // })
                            .or_insert_with(|| grid.is_coord_within_shape(*c, |v, _| *v))
                })
            })
            .map(|(_, _, area)| *area)
            .max()
            .unwrap();

        println!(
            "Cache size: {}",
            cache.len(),
            // cache_entry.load(std::sync::atomic::Ordering::Relaxed) as f64 / cache.len() as f64
        );

        max.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day9::new();
        assert_eq!(
            solution.part1(
                r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
            ),
            String::from("50")
        );
    }

    #[test]
    fn test_part2() {
        let mut solution = Day9::new();
        assert_eq!(
            solution.part2(
                r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
            ),
            String::from("24")
        );
    }
}
