use std::collections::VecDeque;

type XYPoint = (usize, usize);

fn parse_input(input: &str) -> (XYPoint, XYPoint, Vec<Vec<usize>>) {
    let mut position: XYPoint = (0, 0);
    let mut destination: XYPoint = (0, 0);
    let prio = "abcdefghijklmnopqrstuvwxyz";
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        position = (x, y);
                        'a'
                    } else if char == 'E' {
                        destination = (x, y);
                        'z'
                    } else {
                        char
                    }
                })
                .map(|char| prio.chars().position(|c| c == char).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    (position, destination, map)
}

fn steps(start: XYPoint, condition: impl Fn(XYPoint) -> bool, map: &Vec<Vec<usize>>) -> usize {
    let mut queue = VecDeque::from([(25, start)]);
    // Optimization: To check if a position was visited, using an array for direct access is ~5x
    // faster than using a HashSet.
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[start.1][start.0] = true;
    let mut steps = 0;
    while !queue.is_empty() {
        // Empty check rather than pop_front, so we can increase the score by 1 on each iteration
        steps += 1;
        for _ in 0..queue.len() {
            let (height, (x, y)) = queue.pop_front().unwrap();
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let target_x = x as isize + dx;
                let target_y = y as isize + dy;
                if target_x < 0 || target_y < 0 {
                    continue;
                }
                let target_x = target_x as usize;
                let target_y = target_y as usize;
                if target_y >= map.len() || target_x >= map[target_y].len() {
                    continue;
                }
                let target_height = map[target_y][target_x];
                if target_height + 1 < height {
                    continue;
                }
                let visited = &mut visited[target_y][target_x];
                if *visited {
                    continue;
                }
                if condition((target_x, target_y)) {
                    return steps;
                }
                *visited = true;
                queue.push_back((target_height, (target_x, target_y)));
            }
        }
    }
    panic!("Not found")
}

pub fn part1(input: &str) -> Option<usize> {
    let (position, destination, map) = parse_input(input);
    steps(
        destination,
        |(x, y)| x == position.0 && y == position.1,
        &map,
    )
    .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let (_, destination, map) = parse_input(input);
    steps(destination, |(x, y)| map[y][x] == 0, &map).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 12;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(31));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(391));
    } // 409 is too high

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(29));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(386));
    }
}
