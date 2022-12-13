use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(left), Packet::Num(right)) => left.cmp(right),
            (Packet::Num(_), Packet::List(_)) => Packet::List(Vec::from([self.clone()])).cmp(other),
            (Packet::List(_), Packet::Num(_)) => {
                self.cmp(&Packet::List(Vec::from([other.clone()])))
            }
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(string: &str) -> Packet {
    if &string[0..1] == "[" {
        let mut depth: i32 = 0;
        Packet::List(
            string[1..string.len() - 1]
                .split(|c| {
                    if c == '[' {
                        depth += 1
                    } else if c == ']' {
                        depth -= 1
                    }
                    c == ',' && depth == 0
                })
                .filter_map(|string| {
                    if !string.is_empty() {
                        Some(parse_packet(string))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    } else {
        Packet::Num(string.parse().unwrap())
    }
}

pub fn part1(input: &str) -> Option<usize> {
    input
        .split("\n\n")
        .map(|pair| {
            pair.split_once('\n')
                .map(|(left, right)| (parse_packet(left), parse_packet(right)))
                .unwrap()
        })
        .enumerate()
        .filter_map(
            |(index, (left, right))| {
                if left < right {
                    Some(index + 1)
                } else {
                    None
                }
            },
        )
        .sum::<usize>()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let divider1 = parse_packet("[[2]]");
    let divider2 = parse_packet("[[6]]");
    let mut lines = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(parse_packet(line))
            }
        })
        .collect::<Vec<Packet>>();
    lines.insert(0, divider1.clone());
    lines.insert(1, divider2.clone());
    lines.sort();
    let position1 = lines.binary_search(&divider1).unwrap() + 1;
    let position2 = lines.binary_search(&divider2).unwrap() + 1;
    (position1 * position2).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 13;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(13));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(6623));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(140));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(23049));
    }
}
