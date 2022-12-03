use aoc2022::*;
use std::fmt::Display;
use std::time::Instant;

fn main() {
    eval_day(1, day01::part1, day01::part2);
    eval_day(2, day02::part1, day02::part2);
    eval_day(3, day03::part1, day03::part2);
    eval_day(4, day04::part1, day04::part2);
    eval_day(5, day05::part1, day05::part2);
    eval_day(6, day06::part1, day06::part2);
    eval_day(7, day07::part1, day07::part2);
    eval_day(8, day08::part1, day08::part2);
    eval_day(9, day09::part1, day09::part2);
    eval_day(10, day10::part1, day10::part2);
    eval_day(11, day11::part1, day11::part2);
    eval_day(12, day12::part1, day12::part2);
    eval_day(13, day13::part1, day13::part2);
    eval_day(14, day14::part1, day14::part2);
    eval_day(15, day15::part1, day15::part2);
    eval_day(16, day16::part1, day16::part2);
    eval_day(17, day17::part1, day17::part2);
    eval_day(18, day18::part1, day18::part2);
    eval_day(19, day19::part1, day19::part2);
    eval_day(20, day20::part1, day20::part2);
    eval_day(21, day21::part1, day21::part2);
    eval_day(22, day22::part1, day22::part2);
    eval_day(23, day23::part1, day23::part2);
    eval_day(24, day24::part1, day24::part2);
    eval_day(25, day25::part1, day25::part2);
}

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

fn eval_day<T: Display, E: Display>(
    day: usize,
    part1: fn(&str) -> Option<T>,
    part2: fn(&str) -> Option<E>,
) {
    println!("{}🎄Day {}🎄{}", ANSI_BOLD, day, ANSI_RESET);
    let timer = Instant::now();
    let input = utils::read_input(day);
    let elapsed = timer.elapsed();
    println!(
        " {}· Input {}{}(elapsed: {:.2?}){}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, elapsed, ANSI_RESET
    );
    eval_part(1, &input, part1);
    eval_part(2, &input, part2);
}

fn eval_part<T: Display>(part: usize, input: &str, method: fn(&str) -> Option<T>) {
    print!(" {}· Part {}{}: ", ANSI_BOLD, part, ANSI_RESET);
    let timer = Instant::now();
    let result = method(input);
    let elapsed = timer.elapsed();
    match result {
        Some(result) => {
            print!(
                "{} {}(elapsed: {:.2?}){}",
                result, ANSI_ITALIC, elapsed, ANSI_RESET
            );
        }
        None => {
            print!("not solved.")
        }
    }
    println!()
}
