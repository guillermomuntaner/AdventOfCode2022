fn parse_input(
    input: &str,
) -> (
    Vec<Vec<char>>,
    impl Iterator<Item = (usize, usize, usize)> + '_,
) {
    let (cargo_map, instructions) = input.split_once("\n\n").unwrap();
    let mut cargo_rows_iter = cargo_map.lines().rev();
    let num_stacks = cargo_rows_iter
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut cargo = vec![vec![]; num_stacks];
    for cargo_row in cargo_rows_iter {
        for (i, stack) in cargo.iter_mut().enumerate() {
            match cargo_row.chars().nth((i * 4) + 1) {
                Some(char) if char != ' ' => stack.push(char),
                _ => {}
            }
        }
    }

    let instructions = instructions.lines().map(|line| {
        let split = line.split_whitespace().collect::<Vec<_>>();
        (
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap() - 1,
            split[5].parse::<usize>().unwrap() - 1,
        )
    });
    (cargo, instructions)
}

fn top_crates(cargo: &[Vec<char>]) -> Option<String> {
    cargo
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
        .into()
}

pub fn part1(input: &str) -> Option<String> {
    let (mut cargo, instructions) = parse_input(input);

    for (quantity, from, to) in instructions {
        for _ in 0..quantity {
            let moving_crate = cargo[from].pop().unwrap();
            cargo[to].push(moving_crate);
        }
    }

    top_crates(&cargo)
}

pub fn part2(input: &str) -> Option<String> {
    let (mut cargo, instructions) = parse_input(input);

    for (quantity, from, to) in instructions {
        let split_index = cargo[from].len().saturating_sub(quantity);
        let moving_crates = cargo[from].split_off(split_index);
        cargo[to].extend(moving_crates);
    }

    top_crates(&cargo)
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 5;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            super::part1(&utils::read_example(DAY, 1)),
            Some(String::from("CMZ"))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            super::part1(&utils::read_input(DAY)),
            Some(String::from("FWSHSPJWM"))
        );
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(
            super::part2(&utils::read_example(DAY, 1)),
            Some(String::from("MCD"))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            super::part2(&utils::read_input(DAY)),
            Some(String::from("PWPWHGFZS"))
        );
    }
}
