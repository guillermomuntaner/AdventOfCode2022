pub fn start_of_message(input: &str, lenght: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(lenght)
        .enumerate()
        .find_map(|(index, window)| {
            for i in 0..window.len() {
                for j in (i + 1)..window.len() {
                    if window[i] == window[j] {
                        return None;
                    }
                }
            }
            Some(index + lenght)
        })
        .unwrap()
}

pub fn part1(input: &str) -> Option<usize> {
    start_of_message(input, 4).into()
}

pub fn part2(input: &str) -> Option<usize> {
    start_of_message(input, 14).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 6;

    #[test]
    fn test_part1_example1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(super::part1(input), Some(7));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(1275));
    }

    #[test]
    fn test_part1_example2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(super::part2(input), Some(19));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(3605));
    }
}
