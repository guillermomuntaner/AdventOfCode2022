use itertools::Itertools;

pub fn priority(char: &char) -> usize {
    let mut prio = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    1 + prio.position(|c| &c == char).unwrap()
}

pub fn part1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| {
            let (part1, part2) = line.split_at(line.len() / 2);
            part1
                .chars()
                .find(|&char| part2.chars().contains(&char))
                .unwrap()
        })
        .map(|char| priority(&char))
        .sum::<usize>()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(elf1, elf2, elf3)| {
            elf1.chars()
                .find(|&i| elf2.contains(i) && elf3.contains(i))
                .unwrap()
        })
        .map(|char| priority(&char))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 3;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(157));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(7553));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 2)), Some(70));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(2758));
    }
}
