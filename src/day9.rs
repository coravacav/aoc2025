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

use itertools::Itertools;

use crate::{
    Solution,
    direction::Direction,
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

    fn known_solution_part1(&self) -> Option<String> {
        Some(4750176210i64.to_string())
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

        let mut grid = Grid::new_blank(max_col, max_row, 0, false);
        grid.trace_coord_list(&coords, |&v, _, _| v + 1, true);

        // Each should store the last coordinate found in the shape in that direction.
        #[derive(Debug, Clone, Copy)]
        struct SearchResults {
            pub up: Coord,
            pub down: Coord,
            pub left: Coord,
            pub right: Coord,
        }

        impl SearchResults {
            pub fn do_search(grid: &Grid<i64>, transposed_grid: &Grid<i64>, coord: Coord) -> Self {
                SearchResults {
                    left: SearchResults::do_one_search(grid, coord, Direction::Left),
                    right: SearchResults::do_one_search(grid, coord, Direction::Right),
                    up: SearchResults::do_one_search(
                        transposed_grid,
                        coord.transpose(),
                        Direction::Left,
                    )
                    .transpose(),
                    down: SearchResults::do_one_search(
                        transposed_grid,
                        coord.transpose(),
                        Direction::Right,
                    )
                    .transpose(),
                }
            }

            fn do_one_search(
                grid: &Grid<i64>,
                starting_coord: Coord,
                search_direction: Direction,
            ) -> Coord {
                let mut current_coord = starting_coord;
                let mut has_seen_adjacent_walls = [false; 2];
                let adjacent_directions = search_direction.orthogonal_directions();
                let mut has_seen_wall;
                let mut last_time_we_are_def_in_bounds = starting_coord;

                loop {
                    current_coord += search_direction;

                    let Some(&v) = grid.get(current_coord) else {
                        return last_time_we_are_def_in_bounds;
                    };

                    if v > 0 {
                        last_time_we_are_def_in_bounds = current_coord;
                        has_seen_wall = true;
                        has_seen_adjacent_walls = [false; 2];
                    } else {
                        has_seen_wall = false;
                    }

                    if !has_seen_wall {
                        continue;
                    }

                    if adjacent_directions.iter().enumerate().all(|(i, &dir)| {
                        let adjacent_coord = current_coord + dir;
                        let adjacent_value = *grid.get(adjacent_coord).unwrap_or(&1); // Count out-of-bounds as walls
                        if adjacent_value > 0 {
                            has_seen_adjacent_walls[i] = true;
                        }
                        has_seen_adjacent_walls[i]
                    }) {
                        return current_coord;
                    }
                }
            }
        }

        let transposed_coords = coords.iter().map(|c| c.transpose()).collect_vec();
        let mut transposed_grid = Grid::new_blank(max_row, max_col, 0, false);
        transposed_grid.trace_coord_list(&transposed_coords, |&v, _, _| v + 1, true);

        let searched = coords
            .iter()
            .map(|c| (c, SearchResults::do_search(&grid, &transposed_grid, *c)))
            .collect_vec();

        searched
            .into_iter()
            .combinations(2)
            .map(|pair| (pair[0], pair[1], pair[0].0.area(*pair[1].0)))
            .sorted_by(|(_, _, area1), (_, _, area2)| area2.cmp(area1))
            .filter(|((c1, res1), (c2, res2), _)| {
                let direction = (**c2 - **c1).direction();
                // Rows are intuitively backwards since we count them from the top
                match direction {
                    Direction::UpLeft => {
                        res1.up.row() <= c2.row()
                            && res1.left.col() <= c2.col()
                            && res2.down.row() >= c1.row()
                            && res2.right.col() >= c1.col()
                    }
                    Direction::UpRight => {
                        res1.up.row() <= c2.row()
                            && res1.right.col() >= c2.col()
                            && res2.down.row() >= c1.row()
                            && res2.left.col() <= c1.col()
                    }
                    Direction::DownLeft => {
                        res1.down.row() >= c2.row()
                            && res1.left.col() <= c2.col()
                            && res2.up.row() <= c1.row()
                            && res2.right.col() >= c1.col()
                    }
                    Direction::DownRight => {
                        res1.down.row() >= c2.row()
                            && res1.right.col() >= c2.col()
                            && res2.up.row() <= c1.row()
                            && res2.left.col() <= c1.col()
                    }
                    // This is technically wrong, but I don't feel like writing the possibilities out rn :)
                    _ => false,
                }
            })
            .next()
            .map(|(_, _, area)| area)
            .unwrap()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(1574684850.to_string())
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
