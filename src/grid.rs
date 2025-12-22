use std::io::Write;

use image::{EncodableLayout, ImageBuffer, Pixel, PixelWithColorType};
use itertools::Itertools;

use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Coord {
    row: i32,
    col: i32,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare by row first, since that's more cache efficient for iterating
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            other => other,
        }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Coord {
    #[inline(always)]
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    #[inline(always)]
    pub fn new_usize(row: usize, col: usize) -> Self {
        Self::new(i32::try_from(row).unwrap(), i32::try_from(col).unwrap())
    }

    pub fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.row >= 0 && self.row < height && self.col >= 0 && self.col < width
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn is_adjacent(&self, other: Self) -> bool {
        (self.row - 1 == other.row && self.col == other.col)
            || (self.row + 1 == other.row && self.col == other.col)
            || (self.col - 1 == other.col && self.row == other.row)
            || (self.col + 1 == other.col && self.row == other.row)
    }

    pub fn manhattan_distance(&self, other: Self) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }

    pub fn euclidean_distance(&self, other: Self) -> f32 {
        let row_diff = self.row - other.row;
        let col_diff = self.col - other.col;
        ((row_diff * row_diff + col_diff * col_diff) as f32).sqrt()
    }

    pub fn wrap<T>(&self, grid: &Grid<T>) -> Self {
        Self {
            row: self.row.rem_euclid(grid.height),
            col: self.col.rem_euclid(grid.width),
        }
    }

    pub fn normalize_to_direction(&self) -> Direction {
        match (self.row.signum(), self.col.signum()) {
            (0, 1) => Direction::Right,
            (0, -1) => Direction::Left,
            (1, 0) => Direction::Down,
            (-1, 0) => Direction::Up,
            (1, 1) => Direction::DownRight,
            (1, -1) => Direction::DownLeft,
            (-1, 1) => Direction::UpRight,
            (-1, -1) => Direction::UpLeft,
            _ => panic!("Cannot normalize zero vector to direction"),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.row == 0 && self.col == 0
    }

    pub fn area(&self, other: Self) -> i32 {
        let width = (other.col - self.col).abs() + 1;
        let height = (other.row - self.row).abs() + 1;
        width * height
    }

    pub fn other_corners(&self, other: Self) -> [Self; 2] {
        [
            Self::new(self.row, other.col),
            Self::new(other.row, self.col),
        ]
    }

    pub fn all_corners(&self, other: Self) -> [Self; 4] {
        [
            Self::new(self.row, other.col),
            *self,
            Self::new(other.row, self.col),
            other,
        ]
    }

    pub fn coords_on_rectangle_perimeter(&self, other: Self) -> Vec<Coord> {
        let mut coords = vec![];

        let min_row = self.row.min(other.row);
        let max_row = self.row.max(other.row);
        let min_col = self.col.min(other.col);
        let max_col = self.col.max(other.col);

        for col in min_col..=max_col {
            coords.push(Coord::new(min_row, col));
            coords.push(Coord::new(max_row, col));
        }

        for row in (min_row + 1)..(max_row) {
            coords.push(Coord::new(row, min_col));
            coords.push(Coord::new(row, max_col));
        }

        coords
    }

    pub fn coords_in_rectangle_area(&self, other: Self) -> Vec<Coord> {
        let mut coords = vec![];

        let min_row = self.row.min(other.row);
        let max_row = self.row.max(other.row);
        let min_col = self.col.min(other.col);
        let max_col = self.col.max(other.col);

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                coords.push(Coord::new(row, col));
            }
        }

        coords
    }

    pub fn trace_coord_list(coords: &[Coord], close_loop: bool) -> Vec<Coord> {
        if coords.is_empty() {
            return vec![];
        }

        let mut traced_coords = vec![];
        let mut do_trace_step = |mut start: Coord, end: Coord, include_end: bool| {
            assert!(start.row() == end.row() || start.col() == end.col());

            let direction = (end - start).normalize_to_direction();

            while start != end {
                traced_coords.push(start);
                start += direction;
            }

            if include_end {
                traced_coords.push(end);
            }
        };

        for window in coords.windows(2) {
            do_trace_step(window[0], window[1], true);
        }

        if close_loop {
            do_trace_step(coords[coords.len() - 1], coords[0], false);
        }

        traced_coords
    }

    pub fn adjacent_quad_plus(&self) -> [Coord; 4] {
        [
            *self + Direction::Up,
            *self + Direction::Right,
            *self + Direction::Down,
            *self + Direction::Left,
        ]
    }

    pub fn adjacent_quad_cross(&self) -> [Coord; 4] {
        [
            *self + Direction::UpLeft,
            *self + Direction::UpRight,
            *self + Direction::DownLeft,
            *self + Direction::DownRight,
        ]
    }

    pub fn adjacent_octo(&self) -> [Coord; 8] {
        [
            *self + Direction::UpLeft,
            *self + Direction::UpRight,
            *self + Direction::DownLeft,
            *self + Direction::DownRight,
            *self + Direction::Left,
            *self + Direction::Right,
            *self + Direction::Up,
            *self + Direction::Down,
        ]
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, other: Coord) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::Add<i32> for Coord {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            row: self.row + rhs,
            col: self.col + rhs,
        }
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Self;

    fn sub(self, other: Coord) -> Self::Output {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl std::ops::SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.row -= other.row;
        self.col -= other.col;
    }
}

impl std::ops::Mul<i32> for Coord {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self {
            row: self.row * other,
            col: self.col * other,
        }
    }
}

impl std::ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.row += other.row;
        self.col += other.col;
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
    wrapping: bool,
}

impl<T: PartialEq> Grid<T> {
    pub fn new(input: &str, char_to_t: impl Fn(char) -> T, wrapping: bool) -> Self {
        let mut grid = Vec::with_capacity(input.len());
        let mut height = 0;

        for c in input.chars() {
            match c {
                '\n' => {
                    height += 1;
                }
                c => {
                    grid.push(char_to_t(c));
                }
            }
        }

        let ends_in_newline = input.as_bytes().last() == Some(&b'\n');

        if !ends_in_newline {
            height += 1
        };

        // Check if the width = height length makes sense
        let usize_height = height;
        let guess_size = usize_height * usize_height + usize_height - 1;

        let width = if input.len() != guess_size {
            // First length till newline
            let mut width = 0;
            for c in input.chars() {
                if c == '\n' {
                    break;
                }
                width += 1;
            }

            // ensure that all rows have the same length
            assert_eq!(
                (input.len() - height + if ends_in_newline { 0 } else { 1 }) % width,
                0,
                "Grid line width is not consistent"
            );

            width
        } else {
            // assert that it's square
            assert_eq!(input.len(), height * (height + 1) - 1);

            height
        };

        Self {
            width: width as i32,
            height: height as i32,
            data: grid,
            wrapping,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn new_blank(width: i32, height: i32, value: T, wrapping: bool) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![value; width as usize * height as usize],
            wrapping,
        }
    }

    pub fn set_all_coords_to(&mut self, coords: impl Iterator<Item = Coord>, value: T)
    where
        T: Clone,
    {
        for coord in coords {
            self[coord] = value.clone();
        }
    }

    pub fn iter_with_coords(&self) -> impl DoubleEndedIterator<Item = (&T, Coord)> {
        self.data.iter().enumerate().map(|(i, t)| {
            let i = i32::try_from(i).unwrap();

            (t, Coord::new(i / self.width, i % self.width))
        })
    }

    pub fn iter_coords(&self) -> impl DoubleEndedIterator<Item = Coord> {
        self.iter_with_coords().map(|(_, coord)| coord)
    }

    pub fn iter_rows(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        self.data.chunks(self.width as usize)
    }

    pub fn iter_columns_down(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.width).map(|i| {
            self.data
                .iter()
                .skip(i as usize)
                .step_by(self.width as usize)
                .collect()
        })
    }

    pub fn iter_direction_till(
        &self,
        coord: Coord,
        dir: Direction,
        stop_check: impl Fn(Coord) -> bool,
    ) -> impl Iterator<Item = Coord> {
        let mut coord = coord;
        let mut stop = false;

        std::iter::from_fn(move || {
            if stop {
                return None;
            }

            coord += dir;
            if self.wrapping {
                coord = coord.wrap(self);
            }

            if !self.is_coord_in_bounds(coord) || stop_check(coord) {
                stop = true;
                return None;
            }

            Some(coord)
        })
    }

    pub fn search_direction_till(
        &self,
        start: Coord,
        dir: Direction,
        mut search_stop: impl FnMut(&T, Coord) -> bool,
    ) -> bool {
        let mut coord = start;
        loop {
            coord += dir;
            if self.wrapping {
                coord = coord.wrap(self);
            }

            if !self.is_coord_in_bounds(coord) {
                return false;
            }

            if search_stop(&self[coord], coord) {
                return true;
            }
        }
    }

    pub fn iter_area(
        &self,
        one_corner: Coord,
        another_corner: Coord,
    ) -> impl Iterator<Item = (&T, Coord)> {
        let min_row = one_corner.row.min(another_corner.row);
        let max_row = one_corner.row.max(another_corner.row);
        let min_col = one_corner.col.min(another_corner.col);
        let max_col = one_corner.col.max(another_corner.col);

        (min_row..=max_row).flat_map(move |row| {
            (min_col..=max_col).map(move |col| {
                let coord = Coord::new(row, col);
                (&self[coord], coord)
            })
        })
    }

    pub fn is_coord_in_bounds(&self, coord: Coord) -> bool {
        coord.in_bounds(self.width, self.height)
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        let coord = if self.wrapping {
            coord.wrap(self)
        } else {
            coord
        };

        if !self.is_coord_in_bounds(coord) {
            return None;
        }

        Some(&self[coord])
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        let coord = if self.wrapping {
            coord.wrap(self)
        } else {
            coord
        };

        assert!(self.is_coord_in_bounds(coord));

        self[coord] = value;
    }

    pub fn find(&self, val: T) -> Option<Coord> {
        for coord in self.iter_coords() {
            if self[coord] == val {
                return Some(coord);
            }
        }

        None
    }

    pub fn find_all(&self, val: T) -> Vec<Coord> {
        let mut coords = vec![];
        for coord in self.iter_coords() {
            if self[coord] == val {
                coords.push(coord);
            }
        }

        coords
    }

    pub fn find_where(&self, predicate: impl Fn(&T, Coord) -> bool) -> Vec<Coord> {
        let mut coords = vec![];
        for coord in self.iter_coords() {
            if predicate(&self[coord], coord) {
                coords.push(coord);
            }
        }

        coords
    }

    pub fn survey_quad_plus(&self, coord: Coord) -> [Option<&T>; 4] {
        coord.adjacent_quad_plus().map(|c| self.get(c))
    }

    pub fn survey_quad_cross(&self, coord: Coord) -> [Option<&T>; 4] {
        coord.adjacent_quad_cross().map(|c| self.get(c))
    }

    pub fn survey_octo(&self, coord: Coord) -> [Option<&T>; 8] {
        coord.adjacent_octo().map(|c| self.get(c))
    }

    pub fn trace_coord_list(
        &mut self,
        coords: &[Coord],
        setter: impl Fn(&T, Coord, bool) -> T,
        close_loop: bool,
    ) {
        for coord in Coord::trace_coord_list(coords, close_loop) {
            let current_value = self.get(coord).unwrap();
            let new_value = setter(current_value, coord, false);
            self.set(coord, new_value);
        }
    }

    // Implement via raycast even odd method in all four directions
    pub fn is_coord_within_shape(
        &self,
        coord: Coord,
        is_shape_edge: impl Fn(&T, Coord) -> bool,
    ) -> bool {
        // check all four directions
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .all(|&dir| {
            let mut inside = false;
            let mut current = coord;
            loop {
                current += dir;

                if !self.is_coord_in_bounds(current) {
                    break;
                }

                if is_shape_edge(&self[current], current) {
                    inside = !inside;
                    // loop till no longer on edge
                    loop {
                        current += dir;

                        if !self.is_coord_in_bounds(current)
                            || !is_shape_edge(&self[current], current)
                        {
                            break;
                        }
                    }
                }
            }
            inside
        })
    }

    pub fn flood_fill(
        &mut self,
        start: Coord,
        can_flow: impl Fn(&T, Coord) -> bool,
        fill_value: impl Fn(&T, Coord) -> T,
        fail_on_out_of_bounds: bool,
    ) -> bool {
        let mut to_visit = vec![start];
        let mut visited = Vec::with_capacity(self.data.len());

        // first collect all coords to fill
        while let Some(current) = to_visit.pop() {
            if visited
                .binary_search_by(|c: &Coord| c.cmp(&current))
                .is_ok()
            {
                continue;
            }
            visited.push(current);
            visited.sort();
            if !self.is_coord_in_bounds(current) {
                if fail_on_out_of_bounds {
                    return false;
                } else {
                    continue;
                }
            }
            let current_value = self.get(current).unwrap();
            if !can_flow(current_value, current) {
                continue;
            }
            for neighbor in current.adjacent_quad_plus() {
                to_visit.push(neighbor);
            }
        }

        // then fill all collected coords
        for coord in visited {
            let current_value = self.get(coord).unwrap();
            let new_value = fill_value(current_value, coord);
            self.set(coord, new_value);
        }

        true
    }

    pub fn save_grid_to_png<P>(&self, filename: &str, value_to_color: impl Fn(&T, Coord) -> P)
    where
        P: Pixel,
        P: PixelWithColorType,
        [<P as Pixel>::Subpixel]: EncodableLayout,
    {
        // Create a new ImageBuffer
        let img = ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let coord = Coord::new(y as i32, x as i32);
            value_to_color(&self[coord], coord)
        });

        img.save(filename).unwrap();
    }

    pub fn pretty_print_with_printer(&self, w: &mut impl Write, printer: impl Fn(&T) -> String) {
        for line in self.iter_rows() {
            writeln!(w, "{}", line.iter().map(|t| printer(t)).join("")).unwrap();
        }
    }

    pub fn pretty_print_bolded_coord_with_printer(
        &self,
        coord: Coord,
        w: &mut impl Write,
        printer: impl Fn(&T) -> String,
    ) {
        for (i, line) in self.iter_rows().enumerate() {
            writeln!(
                w,
                "{}",
                line.iter()
                    .enumerate()
                    .map(|(j, t)| if Coord::new_usize(i, j) == coord {
                        use colored::*;
                        printer(t).on_bright_red().black().to_string()
                    } else {
                        printer(t)
                    })
                    .join("")
            )
            .unwrap();
        }
    }

    pub fn pretty_print_bolded_coords_with_printer(
        &self,
        coords: &[Coord],
        w: &mut impl Write,
        printer: impl Fn(&T) -> String,
    ) {
        for (i, line) in self.iter_rows().enumerate() {
            writeln!(
                w,
                "{}",
                line.iter()
                    .enumerate()
                    .map(|(j, t)| if coords.contains(&Coord::new_usize(i, j)) {
                        use colored::*;
                        printer(t).on_bright_red().black().to_string()
                    } else {
                        printer(t)
                    })
                    .join("")
            )
            .unwrap();
        }
    }

    pub fn pretty_print_bolded_coords_area_with_printer(
        &self,
        one_corner: Coord,
        another_corner: Coord,
        w: &mut impl Write,
        printer: impl Fn(&T) -> String,
    ) {
        let min_row = one_corner.row.min(another_corner.row);
        let max_row = one_corner.row.max(another_corner.row);
        let min_col = one_corner.col.min(another_corner.col);
        let max_col = one_corner.col.max(another_corner.col);

        for (i, line) in self.iter_rows().enumerate() {
            writeln!(
                w,
                "{}",
                line.iter()
                    .enumerate()
                    .map(|(j, t)| {
                        if i as i32 >= min_row
                            && i as i32 <= max_row
                            && j as i32 >= min_col
                            && j as i32 <= max_col
                        {
                            use colored::*;
                            printer(t).on_bright_red().black().to_string()
                        } else {
                            printer(t)
                        }
                    })
                    .join("")
            )
            .unwrap();
        }
    }

    pub fn pretty_print_low_resolution_with_printer(
        &self,
        height_factor: usize,
        width_factor: usize,
        w: &mut impl Write,
        printer: impl Fn(&[(&T, Coord)]) -> String,
    ) {
        assert!(height_factor >= self.height as usize);
        assert!(width_factor >= self.width as usize);

        let mut height_chunks = Vec::new();
        let mut width_chunks = Vec::new();

        for row_start in (0..self.height).step_by(height_factor) {
            let row_end = (row_start + height_factor as i32).min(self.height);
            height_chunks.push(row_start..row_end);
        }

        for col_start in (0..self.width).step_by(width_factor) {
            let col_end = (col_start + width_factor as i32).min(self.width);
            width_chunks.push(col_start..col_end);
        }

        let mut line_parts = vec![];
        let mut block_values = vec![];

        for row_range in height_chunks {
            line_parts.clear();
            for col_range in &width_chunks {
                block_values.clear();
                for row in row_range.clone() {
                    for col in col_range.clone() {
                        let coord = Coord::new(row as i32, col as i32);
                        block_values.push((&self[coord], coord));
                    }
                }
                line_parts.push(printer(&block_values));
            }
            writeln!(w, "{}", line_parts.join("")).unwrap();
        }
    }

    pub fn pretty_print_low_resolution_square_with_printer(
        &self,
        square_size: usize,
        w: &mut impl Write,
        printer: impl Fn(&[(&T, Coord)]) -> String,
    ) {
        let height = self.height as usize;
        let width = self.width as usize;
        let height_factor = if height % square_size == 0 {
            height / square_size
        } else {
            height / square_size + 1
        };

        let width_factor = if width % square_size == 0 {
            width / square_size
        } else {
            width / square_size + 1
        };

        self.pretty_print_low_resolution_with_printer(height_factor, width_factor, w, printer);
    }
}

impl<T: std::fmt::Display + PartialEq> Grid<T> {
    pub fn pretty_print(&self, w: &mut impl Write) {
        self.pretty_print_with_printer(w, T::to_string);
    }

    pub fn pretty_print_bolded_coord(&self, coord: Coord, w: &mut impl Write) {
        self.pretty_print_bolded_coord_with_printer(coord, w, T::to_string);
    }

    pub fn pretty_print_bolded_coords(&self, coords: &[Coord], w: &mut impl Write) {
        self.pretty_print_bolded_coords_with_printer(coords, w, T::to_string);
    }

    pub fn pretty_print_bolded_coords_area(
        &self,
        one_corner: Coord,
        another_corner: Coord,
        w: &mut impl Write,
    ) {
        self.pretty_print_bolded_coords_area_with_printer(
            one_corner,
            another_corner,
            w,
            T::to_string,
        );
    }
}

impl<T> std::ops::Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.data[coord.col as usize + coord.row as usize * self.width as usize]
    }
}

impl<T> std::ops::IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.col as usize + index.row as usize * self.width as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_iter_with_coords() {
        let input = "abc\ndef\nghi";
        let grid = Grid::new(input, |c| c, false);

        let mut coords_and_values = grid.iter_with_coords();

        assert_eq!(coords_and_values.next(), Some((&'a', Coord::new(0, 0))));
        assert_eq!(coords_and_values.next(), Some((&'b', Coord::new(0, 1))));
        assert_eq!(coords_and_values.next(), Some((&'c', Coord::new(0, 2))));
        assert_eq!(coords_and_values.next(), Some((&'d', Coord::new(1, 0))));
        assert_eq!(coords_and_values.next(), Some((&'e', Coord::new(1, 1))));
        assert_eq!(coords_and_values.next(), Some((&'f', Coord::new(1, 2))));
        assert_eq!(coords_and_values.next(), Some((&'g', Coord::new(2, 0))));
        assert_eq!(coords_and_values.next(), Some((&'h', Coord::new(2, 1))));
        assert_eq!(coords_and_values.next(), Some((&'i', Coord::new(2, 2))));
        assert_eq!(coords_and_values.next(), None);
    }

    #[test]
    fn test_dims() {
        let input = "abc\ndef";
        let grid = Grid::new(input, |c| c, false);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);

        let input = "abc\ndef\n";
        let grid = Grid::new(input, |c| c, false);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);

        let input2 = "abc \nefgh\nijkl\nmnop\n";
        let grid2 = Grid::new(input2, |c| c, false);
        assert_eq!(grid2.width(), 4);
        assert_eq!(grid2.height(), 4);
    }

    #[test]
    fn test_flood_fill() {
        let coords = [
            Coord::new(1, 7),
            Coord::new(1, 11),
            Coord::new(7, 11),
            Coord::new(7, 9),
            Coord::new(5, 9),
            Coord::new(5, 2),
            Coord::new(3, 2),
            Coord::new(3, 7),
        ];

        let mut grid = Grid::new_blank(12, 8, false, false);
        grid.trace_coord_list(&coords, |_, _, _| true, true);

        let mut v = Vec::new();
        grid.pretty_print_with_printer(&mut v, |v| if *v { '#' } else { '.' }.to_string());

        assert_eq!(
            str::from_utf8(&v).unwrap(),
            r#"
............
.......#####
.......#...#
..######...#
..#........#
..########.#
.........#.#
.........###
"#
            .trim_start()
        );

        grid.flood_fill(Coord::new(2, 10), |v, _| !v, |_, _| true, true);

        let mut v = Vec::new();
        grid.pretty_print_with_printer(&mut v, |v| if *v { '#' } else { '.' }.to_string());

        assert_eq!(
            str::from_utf8(&v).unwrap(),
            r#"
............
.......#####
.......#####
..##########
..##########
..##########
.........###
.........###
"#
            .trim_start()
        );
    }

    #[test]
    fn test_is_coord_within_shape() {
        let coords = [
            Coord::new(1, 7),
            Coord::new(1, 11),
            Coord::new(7, 11),
            Coord::new(7, 9),
            Coord::new(5, 9),
            Coord::new(5, 2),
            Coord::new(3, 2),
            Coord::new(3, 7),
        ];

        let mut grid = Grid::new_blank(12, 8, false, false);
        grid.trace_coord_list(&coords, |_, _, _| true, true);

        let outline: HashSet<Coord> = HashSet::from_iter(grid.find_all(true));

        let mut flooded_grid = grid.clone();

        flooded_grid.flood_fill(Coord::new(2, 10), |v, _| !v, |_, _| true, true);

        let all = HashSet::from_iter(flooded_grid.find_all(true));
        let inner = all.difference(&outline);
        for coord in inner {
            if !grid.is_coord_within_shape(*coord, |v, _| *v) {
                grid.pretty_print_bolded_coord_with_printer(*coord, &mut std::io::stderr(), |v| {
                    if *v { '#' } else { '.' }.to_string()
                });

                panic!("Coord {} should be inside shape", coord);
            }
        }
    }
}
