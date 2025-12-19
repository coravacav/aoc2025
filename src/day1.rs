use crate::Solution;

#[derive(Default)]
struct Val(u16);

impl Val {
    fn get(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    fn set(&mut self, n: u8) {
        self.0 = 1 << n;
    }

    fn new(n: u8) -> Self {
        let mut new = Val::default();
        new.set(n);
        new
    }
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get().to_string())
    }
}

pub struct Day1 {}

fn recur_count_solve(mut rem: &[u8], digits_left: u8) -> i64 {
    if digits_left == 0 {
        return 0;
    }

    let (best_index, best_number) = rem[..rem.len() - digits_left as usize + 1]
        .iter()
        .enumerate()
        .fold(
            (0, 0),
            |(best_i, best_n), (i, &n)| {
                if n > best_n { (i, n) } else { (best_i, best_n) }
            },
        );

    (best_number as i64) * 10_i64.pow(digits_left as u32 - 1)
        + recur_count_solve(&rem[best_index + 1..], digits_left - 1)
}

impl Solution for Day1 {
    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .trim()
            .lines()
            .map(|line| {
                recur_count_solve(
                    &line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>(),
                    2,
                )
            })
            .sum::<i64>()
            .to_string()
    }

    fn known_solution_part1(&self) -> Option<String> {
        Some(String::from("17493"))
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .trim()
            .lines()
            .map(|line| {
                recur_count_solve(
                    &line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>(),
                    12,
                )
            })
            .sum::<i64>()
            .to_string()
    }

    fn known_solution_part2(&self) -> Option<String> {
        Some(String::from("173685428989126"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut solution = Day1::new();
        assert_eq!(
            solution.part1(
                r#"987654321111111
811111111111119
234234234234278
818181911112111"#
            ),
            String::from("357")
        );
    }

    //     #[test]
    //     fn test_part2() {
    //         let mut solution = Day1::new();
    //         assert_eq!(
    //             solution.part2(
    //                 r#"3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3"#
    //             ),
    //             String::from("31")
    //         );
    //     }
}
