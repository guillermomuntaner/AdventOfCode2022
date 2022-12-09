fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|instruction| {
            instruction
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn part1(input: &str) -> Option<usize> {
    let rows = parse_input(input);
    let (height, width) = (rows.len(), rows[0].len());
    rows.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, &current)| {
                    (0..y).all(|y1| rows[y1][*x] < current)
                        || ((y + 1)..height).all(|y1| rows[y1][*x] < current)
                        || (0..*x).all(|x1| rows[y][x1] < current)
                        || ((x + 1)..width).all(|x1| rows[y][x1] < current)
                })
                .count()
        })
        .sum::<usize>()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let rows = parse_input(input);
    let (height, width) = (rows.len(), rows[0].len());
    rows.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &current)| {
                    // Need to differentiate between when there is nothing, score 0, and when there is a
                    // tree, score X. Because rust doesn't have a "take_until" operator, its a bit ugly :|
                    let top = 0..y;
                    let mut bottom = (y + 1)..height;
                    let left = 0..x;
                    let mut right = (x + 1)..width;
                    if top.is_empty() || bottom.is_empty() || left.is_empty() || right.is_empty() {
                        0
                    } else {
                        let (top_len, bottom_len, left_len, right_len) =
                            (top.len(), bottom.len(), left.len(), right.len());
                        top.rev()
                            .position(|y1| rows[y1][x] >= current)
                            .map(|a| a + 1)
                            .unwrap_or(top_len)
                            * bottom
                                .position(|y1| rows[y1][x] >= current)
                                .map(|a| a + 1)
                                .unwrap_or(bottom_len)
                            * left
                                .rev()
                                .position(|x1| rows[y][x1] >= current)
                                .map(|a| a + 1)
                                .unwrap_or(left_len)
                            * right
                                .position(|x1| rows[y][x1] >= current)
                                .map(|a| a + 1)
                                .unwrap_or(right_len)
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 8;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(21));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(1801));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(8));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(209880));
    }
}
