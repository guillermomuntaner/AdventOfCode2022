pub fn part1(input: &str) -> Option<usize> {
    input
        .split("\n\n")
        .map(|elf| {
            return elf
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>();
        })
        .max()
        .unwrap()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let mut elfs: Vec<usize> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect();
    elfs.sort_by(|a, b| b.cmp(a));
    return elfs.iter().take(3).sum::<usize>().into();
}

#[cfg(test)]
mod tests {
    use crate::utils;
    const DAY: usize = 1;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(24000));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(68775));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(45000));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(202585));
    }
}
