use itertools::Itertools;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(parse_line)
        .filter(|(elf1, elf2)| {
            (elf1.contains(elf2.start()) && elf1.contains(elf2.end()))
                || (elf2.contains(elf1.start()) && elf2.contains(elf1.end()))
        })
        .count()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .lines()
        .map(parse_line)
        .filter(|(elf1, elf2)| {
            (elf1.contains(elf2.start()) || elf1.contains(elf2.end()))
                || (elf2.contains(elf1.start()) || elf2.contains(elf1.end()))
        })
        .count()
        .into()
}

fn parse_line(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    line.split(',')
        .map(|elf| {
            let mut iter = elf.split('-').map(|num| num.parse::<usize>().unwrap());
            iter.next().unwrap()..=iter.next().unwrap()
        })
        .tuples::<(_, _)>()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 4;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(475));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(4));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(825));
    }
}
