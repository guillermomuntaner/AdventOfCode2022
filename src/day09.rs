use std::collections::HashSet;

fn parse_input(input: &str) -> impl Iterator<Item = (&str, usize)> + '_ {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(instruction, value)| (instruction, value.parse::<usize>().unwrap()))
}

fn rope_simulator<'a>(
    length: usize,
    instructions: impl Iterator<Item = (&'a str, usize)>,
) -> usize {
    let mut tail_visited: HashSet<(isize, isize)> = HashSet::new();
    let mut rope = vec![(0_isize, 0_isize); length];
    tail_visited.insert((0, 0));
    for (instruction, value) in instructions {
        let offset = match instruction {
            "U" => (1, 0),
            "D" => (-1, 0),
            "R" => (0, 1),
            "L" => (0, -1),
            _ => panic!("Unexpected instruction {}", instruction),
        };
        'iter: for _ in 0..value {
            for i in 0..rope.len() {
                if i == 0 {
                    rope[i] = (rope[0].0 + offset.0, rope[0].1 + offset.1)
                } else {
                    let (head_y, head_x) = rope[i - 1];
                    let (tail_y, tail_x) = rope[i];
                    if (head_y - tail_y).abs() > 1 || (head_x - tail_x).abs() > 1 {
                        rope[i] = (
                            tail_y + (head_y - tail_y).signum(),
                            tail_x + (head_x - tail_x).signum(),
                        );
                        if i == rope.len() - 1 {
                            tail_visited.insert(rope[i]);
                        }
                    } else {
                        // Optimization: If a knot doesn't move all subsequent won't either
                        continue 'iter;
                    }
                }
            }
        }
    }
    tail_visited.len()
}

pub fn part1(input: &str) -> Option<usize> {
    let instructions = parse_input(input);
    rope_simulator(2, instructions).into()
}

pub fn part2(input: &str) -> Option<usize> {
    let instructions = parse_input(input);
    rope_simulator(10, instructions).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 9;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(13));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(6269));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(2557));
    }
}
