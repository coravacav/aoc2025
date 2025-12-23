use aoc2025::{get_input, get_solution};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What day of the AoC is this?
    num: u8,
    /// Whether to time the solution parts
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut start_part1 = None;
    let mut start_part2 = None;

    let input = get_input(cli.num);
    let input = input.trim();
    let mut solution = get_solution(cli.num);

    if cli.time {
        start_part1 = Some(std::time::Instant::now());
    }

    let part1 = solution.part1(input);
    if let Some(known_part1) = solution.known_solution_part1() {
        assert_eq!(part1, known_part1);
    }

    if let Some(start_part1) = start_part1 {
        println!("Part 1 took {:?}", start_part1.elapsed());
    }

    println!("Part 1: {}", part1);

    if cli.time {
        start_part2 = Some(std::time::Instant::now());
    }

    let part2 = solution.part2(input);
    if let Some(known_part2) = solution.known_solution_part2() {
        assert_eq!(part2, known_part2);
    }

    if let Some(start_part2) = start_part2 {
        println!("Part 2 took {:?}", start_part2.elapsed());
    }

    println!("Part 2: {}", part2);
}
