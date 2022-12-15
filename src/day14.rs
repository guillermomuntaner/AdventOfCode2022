use itertools::Itertools;
use std::cmp::{max, min};

type XYPoint = (usize, usize);

#[allow(clippy::needless_range_loop)]
fn parse_input(input: &str) -> (usize, Vec<Vec<char>>) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let rocks = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let point = point
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap();
                    if point.0 < min_x {
                        min_x = point.0
                    }
                    if point.0 > max_x {
                        max_x = point.0
                    }
                    if point.1 > max_y {
                        max_y = point.1
                    }
                    point
                })
                .tuple_windows()
                .collect::<Vec<(XYPoint, XYPoint)>>()
        })
        .collect::<Vec<_>>();

    let mut map = vec![vec![' '; max_x - min_x + 1]; max_y + 1];
    for (from, to) in rocks {
        if from.0 == to.0 {
            for y in min(from.1, to.1)..=max(from.1, to.1) {
                map[y][from.0 - min_x] = '#';
            }
        } else if from.1 == to.1 {
            for x in min(from.0, to.0)..=max(from.0, to.0) {
                map[from.1][x - min_x] = '#';
            }
        } else {
            panic!("Unexpected diagonal")
        }
    }
    (min_x, map)
}

fn pour_sand(start_x: usize, inf_map: &mut Vec<Vec<char>>) -> usize {
    let mut sand_count = 0;
    'outer: loop {
        let (mut x, mut y) = (start_x, 0);
        loop {
            if y + 1 >= inf_map.len() {
                break 'outer;
            } else if inf_map[y + 1][x] == ' ' {
                y += 1;
                continue;
            } else if x == 0 {
                break 'outer;
            } else if inf_map[y + 1][x - 1] == ' ' {
                y += 1;
                x -= 1;
            } else if x == inf_map[0].len() - 1 {
                break 'outer;
            } else if inf_map[y + 1][x + 1] == ' ' {
                y += 1;
                x += 1;
            } else {
                sand_count += 1;
                inf_map[y][x] = 'o';
                if y == 0 {
                    break 'outer;
                }
                break;
            }
        }
    }
    sand_count
}

pub fn part1(input: &str) -> Option<usize> {
    let (min_x, mut map) = parse_input(input);
    pour_sand(500 - min_x, &mut map).into()
}

pub fn part2(input: &str) -> Option<usize> {
    let (min_x, map) = parse_input(input);
    // Because diagonal propagation, worst case would be the height in both directions.
    let height = map.len();
    let mut inf_map = map
        .iter()
        .map(|row| {
            let mut vec = vec![' '; height];
            vec.extend(row.iter());
            vec.extend(vec![' '; height].iter());
            vec
        })
        .collect::<Vec<_>>();
    // Add empty row & floor row
    inf_map.push(vec![' '; inf_map[0].len()]);
    inf_map.push(vec!['#'; inf_map[0].len()]);
    pour_sand(500 - min_x + height, &mut inf_map).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 14;

    #[test]
    fn test_part1_example1() {
        let input = "\
            498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9\
            ";
        assert_eq!(super::part1(input), Some(24));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(737));
    }

    #[test]
    fn test_part2_example1() {
        let input = "\
            498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9\
            ";
        assert_eq!(super::part2(input), Some(93));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(28145));
    }
}
