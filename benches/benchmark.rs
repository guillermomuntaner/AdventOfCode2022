use aoc2022::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

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
    let input = black_box(utils::read_input(day));
    match part1(&input) {
        Some(_) => {
            c.bench_with_input(
                BenchmarkId::new(format!("Day {:2}, Part 1", day), day),
                &input,
                |b, input| b.iter(|| part1(input)),
            );
        }
        None => {
            println!("Day {:2}, Part 1 not solved", day)
        }
    }
    match part2(&input) {
        Some(_) => {
            c.bench_with_input(
                BenchmarkId::new(format!("Day {:2}, Part 2", day), day),
                &input,
                |b, input| b.iter(|| part2(input)),
            );
        }
        None => {
            println!("Day {:2}, Part 2 not solved", day)
        }
    }
}

criterion_group!(
    benchmark,
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
