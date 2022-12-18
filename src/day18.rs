use itertools::Itertools;
use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|component| component.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn neighbours((x, y, z): (isize, isize, isize)) -> [(isize, isize, isize); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn part1(input: &str) -> Option<usize> {
    let scan = parse_input(input);
    scan.iter()
        .flat_map(|&position| neighbours(position))
        .filter(|neighbour| !scan.contains(neighbour))
        .count()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let scan = parse_input(input);

    let max = scan.iter().flat_map(|&(x, y, z)| [x, y, z]).max().unwrap() + 1;
    let mut visited = HashSet::new();
    let mut stack = vec![(0, 0, 0)];
    while let Some(position) = stack.pop() {
        for neighbour in neighbours(position) {
            if !scan.contains(&neighbour)
                && !visited.contains(&neighbour)
                && [neighbour.0, neighbour.1, neighbour.2]
                    .iter()
                    .all(|&i| -1 <= i && i <= max)
            {
                visited.insert(neighbour);
                stack.push(neighbour);
            }
        }
    }

    scan.iter()
        .flat_map(|&position| neighbours(position))
        .filter(|neighbour| visited.contains(neighbour))
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 18;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(64));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(4242));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(58));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(2428));
    }
}
