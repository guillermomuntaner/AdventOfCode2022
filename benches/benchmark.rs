use aoc2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn all_days(c: &mut Criterion) {
    c.bench_function("All days", |b| {
        b.iter(|| {
            let day01_input = utils::read_input(1);
            day01::part1(&day01_input);
            day01::part2(&day01_input);
            let day02_input = utils::read_input(2);
            day02::part1(&day02_input);
            day02::part2(&day02_input);
            let day03_input = utils::read_input(3);
            day03::part1(&day03_input);
            day03::part2(&day03_input);
            let day04_input = utils::read_input(4);
            day04::part1(&day04_input);
            day04::part2(&day04_input);
            let day05_input = utils::read_input(5);
            day05::part1(&day05_input);
            day05::part2(&day05_input);
            let day06_input = utils::read_input(6);
            day06::part1(&day06_input);
            day06::part2(&day06_input);
            let day07_input = utils::read_input(7);
            day07::part1(&day07_input);
            day07::part2(&day07_input);
            let day08_input = utils::read_input(8);
            day08::part1(&day08_input);
            day08::part2(&day08_input);
            let day09_input = utils::read_input(9);
            day09::part1(&day09_input);
            day09::part2(&day09_input);
            let day10_input = utils::read_input(10);
            day10::part1(&day10_input);
            day10::part2(&day10_input);
            let day11_input = utils::read_input(11);
            day11::part1(&day11_input);
            day11::part2(&day11_input);
            let day12_input = utils::read_input(12);
            day12::part1(&day12_input);
            day12::part2(&day12_input);
            let day13_input = utils::read_input(13);
            day13::part1(&day13_input);
            day13::part2(&day13_input);
            let day14_input = utils::read_input(14);
            day14::part1(&day14_input);
            day14::part2(&day14_input);
            let day15_input = utils::read_input(15);
            day15::part1(&day15_input);
            day15::part2(&day15_input);
            let day16_input = utils::read_input(16);
            day16::part1(&day16_input);
            day16::part2(&day16_input);
            let day17_input = utils::read_input(17);
            day17::part1(&day17_input);
            day17::part2(&day17_input);
            let day18_input = utils::read_input(18);
            day18::part1(&day18_input);
            day18::part2(&day18_input);
            let day19_input = utils::read_input(19);
            day19::part1(&day19_input);
            day19::part2(&day19_input);
            let day20_input = utils::read_input(20);
            day20::part1(&day20_input);
            day20::part2(&day20_input);
            let day21_input = utils::read_input(21);
            day21::part1(&day21_input);
            day21::part2(&day21_input);
            let day22_input = utils::read_input(22);
            day22::part1(&day22_input);
            day22::part2(&day22_input);
            let day23_input = utils::read_input(23);
            day23::part1(&day23_input);
            day23::part2(&day23_input);
            let day24_input = utils::read_input(24);
            day24::part1(&day24_input);
            day24::part2(&day24_input);
            let day25_input = utils::read_input(25);
            day25::part1(&day25_input);
            day25::part2(&day25_input);
        })
    });
}
fn day01_benchmark(c: &mut Criterion) {
    day_benchmark(c, 1, day01::part1, day01::part2)
}
fn day02_benchmark(c: &mut Criterion) {
    day_benchmark(c, 2, day02::part1, day02::part2)
}
fn day03_benchmark(c: &mut Criterion) {
    day_benchmark(c, 3, day03::part1, day03::part2)
}
fn day04_benchmark(c: &mut Criterion) {
    day_benchmark(c, 4, day04::part1, day04::part2)
}
fn day05_benchmark(c: &mut Criterion) {
    day_benchmark(c, 5, day05::part1, day05::part2)
}
fn day06_benchmark(c: &mut Criterion) {
    day_benchmark(c, 6, day06::part1, day06::part2)
}
fn day07_benchmark(c: &mut Criterion) {
    day_benchmark(c, 7, day07::part1, day07::part2)
}
fn day08_benchmark(c: &mut Criterion) {
    day_benchmark(c, 8, day08::part1, day08::part2)
}
fn day09_benchmark(c: &mut Criterion) {
    day_benchmark(c, 9, day09::part1, day09::part2)
}
fn day10_benchmark(c: &mut Criterion) {
    day_benchmark(c, 10, day10::part1, day10::part2)
}
fn day11_benchmark(c: &mut Criterion) {
    day_benchmark(c, 11, day11::part1, day11::part2)
}
fn day12_benchmark(c: &mut Criterion) {
    day_benchmark(c, 12, day12::part1, day12::part2)
}
fn day13_benchmark(c: &mut Criterion) {
    day_benchmark(c, 13, day13::part1, day13::part2)
}
fn day14_benchmark(c: &mut Criterion) {
    day_benchmark(c, 14, day14::part1, day14::part2)
}
fn day15_benchmark(c: &mut Criterion) {
    day_benchmark(c, 15, day15::part1, day15::part2)
}
fn day16_benchmark(c: &mut Criterion) {
    day_benchmark(c, 16, day16::part1, day16::part2)
}
fn day17_benchmark(c: &mut Criterion) {
    day_benchmark(c, 17, day17::part1, day17::part2)
}
fn day18_benchmark(c: &mut Criterion) {
    day_benchmark(c, 18, day18::part1, day18::part2)
}
fn day19_benchmark(c: &mut Criterion) {
    day_benchmark(c, 19, day19::part1, day19::part2)
}
fn day20_benchmark(c: &mut Criterion) {
    day_benchmark(c, 20, day20::part1, day20::part2)
}
fn day21_benchmark(c: &mut Criterion) {
    day_benchmark(c, 21, day21::part1, day21::part2)
}
fn day22_benchmark(c: &mut Criterion) {
    day_benchmark(c, 22, day22::part1, day22::part2)
}
fn day23_benchmark(c: &mut Criterion) {
    day_benchmark(c, 23, day23::part1, day23::part2)
}
fn day24_benchmark(c: &mut Criterion) {
    day_benchmark(c, 24, day24::part1, day24::part2)
}
fn day25_benchmark(c: &mut Criterion) {
    day_benchmark(c, 25, day25::part1, day25::part2)
}

fn day_benchmark<T, E>(
    c: &mut Criterion,
    day: usize,
    part1: fn(&str) -> Option<T>,
    part2: fn(&str) -> Option<E>,
) {
    match part1(&utils::read_input(day)) {
        Some(_) => {
            c.bench_function(&format!("Day {:0>2}, Part 1", day), |b| {
                b.iter(|| part1(&utils::read_input(day)))
            });
        }
        None => {
            println!("Day {:2}, Part 1 not solved", day)
        }
    }
    match part2(&utils::read_input(day)) {
        Some(_) => {
            c.bench_function(&format!("Day {:0>2}, Part 2", day), |b| {
                b.iter(|| part2(&utils::read_input(day)))
            });
        }
        None => {
            println!("Day {:2}, Part 2 not solved", day)
        }
    }
}

criterion_group!(
    benchmark,
    all_days,
    day01_benchmark,
    day02_benchmark,
    day03_benchmark,
    day04_benchmark,
    day05_benchmark,
    day06_benchmark,
    day07_benchmark,
    day08_benchmark,
    day09_benchmark,
    day10_benchmark,
    day11_benchmark,
    day12_benchmark,
    day13_benchmark,
    day14_benchmark,
    day15_benchmark,
    day16_benchmark,
    day17_benchmark,
    day18_benchmark,
    day19_benchmark,
    day20_benchmark,
    day21_benchmark,
    day22_benchmark,
    day23_benchmark,
    day24_benchmark,
    day25_benchmark,
);
criterion_main!(benchmark);
