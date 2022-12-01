pub fn part1(_input: &str) -> Option<usize> {
    None
}

pub fn part2(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 12;

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), None);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), None);
    }
}
