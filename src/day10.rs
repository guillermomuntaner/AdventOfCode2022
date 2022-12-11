use itertools::Itertools;

pub fn part1(input: &str) -> Option<i32> {
    let mut instructions = input.lines();
    let mut cycle = 1;
    let mut x = 1;
    let mut pending_addx_op: Option<i32> = None;
    let mut signal = 0;
    loop {
        if (cycle - 20) % 40 == 0 {
            signal += x * cycle;
        }
        match pending_addx_op {
            Some(value) => {
                x += value;
                pending_addx_op = None;
            }
            None => match instructions.next() {
                None => break,
                Some(instruction) => {
                    match &instruction[0..4] {
                        "noop" => {}
                        "addx" => pending_addx_op = Some(instruction[5..].parse::<i32>().unwrap()),
                        _ => panic!("Unexpected instruction code"),
                    };
                }
            },
        };
        cycle += 1;
    }
    signal.into()
}

pub fn part2(input: &str) -> Option<String> {
    let mut instructions = input.lines();

    let crt_width = 40;
    let crt_height = 6;

    let mut cycle = 1;
    let mut x = 1;
    let mut pending_addx_op: Option<i32> = None;

    let mut crt = vec![vec!['.'; crt_width]; crt_height];

    loop {
        let crt_x_position = (cycle - 1) % crt_width;
        let crt_y_position = (cycle - 1) / crt_width;
        if ((x - 1)..=(x + 1)).contains(&(crt_x_position as isize)) {
            crt[crt_y_position][crt_x_position] = '#';
        }

        match pending_addx_op {
            Some(value) => {
                x = (x as i32 + value) as isize;
                pending_addx_op = None;
            }
            None => match instructions.next() {
                None => break,
                Some(instruction) => {
                    match &instruction[0..4] {
                        "noop" => {}
                        "addx" => pending_addx_op = Some(instruction[5..].parse::<i32>().unwrap()),
                        _ => panic!("Unexpected instruction code"),
                    };
                }
            },
        };
        cycle += 1;
    }

    crt.iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
        .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 10;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(13140));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(14060));
    }

    #[test]
    fn test_part2_example1() {
        let output = String::from(
            "\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....",
        );
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(output));
    }

    #[test]
    fn test_part2() {
        let output = String::from(
            "\
        ###...##..###..#..#.####.#..#.####...##.\n\
        #..#.#..#.#..#.#.#..#....#.#..#.......#.\n\
        #..#.#..#.#..#.##...###..##...###.....#.\n\
        ###..####.###..#.#..#....#.#..#.......#.\n\
        #....#..#.#....#.#..#....#.#..#....#..#.\n\
        #....#..#.#....#..#.#....#..#.####..##..",
        );
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(output));
    }
}
