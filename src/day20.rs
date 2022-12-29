fn parse_input(input: &str, decryption_key: isize) -> Vec<(usize, isize)> {
    let file = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(|number| number * decryption_key)
        .enumerate()
        .collect::<Vec<_>>();
    file
}

fn mix(file: &mut Vec<(usize, isize)>, initial: &[(usize, isize)]) {
    for (original_index, number) in initial.iter() {
        let current_pos = file
            .iter()
            .position(|(index, _)| original_index == index)
            .unwrap();
        let moving = file.remove(current_pos);
        let target_position =
            ((current_pos as isize + number).rem_euclid(file.len() as isize)) as usize;
        file.insert(target_position, moving);
    }
}

fn grove_coordinate(file: Vec<(usize, isize)>) -> isize {
    let initial_pos = file.iter().position(|(_, number)| *number == 0).unwrap();
    file.get((initial_pos + 1000) % file.len()).unwrap().1
        + file.get((initial_pos + 2000) % file.len()).unwrap().1
        + file.get((initial_pos + 3000) % file.len()).unwrap().1
}

pub fn part1(input: &str) -> Option<isize> {
    let file = parse_input(input, 1);
    let mut mixed = file.clone();
    mix(&mut mixed, &file);
    grove_coordinate(mixed).into()
}

pub fn part2(input: &str) -> Option<isize> {
    let file = parse_input(input, 811589153);
    let mut mixed = file.clone();
    for _ in 0..10 {
        mix(&mut mixed, &file);
    }
    grove_coordinate(mixed).into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 20;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(3));
    }

    #[test]
    fn test_part1() {
        // -5067 -> WRONG
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(19070));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(1623178306));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(14773357352059));
    }
}
