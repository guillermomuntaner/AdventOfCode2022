use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Operation {
    Sum,
    Multiplication,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    left: Option<usize>,
    operation: Operation,
    right: Option<usize>,
    test_divisible: usize,
    case_true: usize,
    case_false: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_input| {
            let mut iter = monkey_input.lines().skip(1);
            let items = iter
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<VecDeque<_>>();
            let mut ops = iter
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = ")
                .unwrap()
                .split(' ');
            let (a, operation, b) = (
                ops.next().unwrap(),
                ops.next().unwrap(),
                ops.next().unwrap(),
            );
            Monkey {
                items,
                left: if a != "old" { a.parse().ok() } else { None },
                operation: if operation == "+" {
                    Operation::Sum
                } else if operation == "*" {
                    Operation::Multiplication
                } else {
                    panic!("Unexpected op {}", operation)
                },
                right: if b != "old" { b.parse().ok() } else { None },
                test_divisible: iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
                case_true: iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
                case_false: iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
            }
        })
        .collect()
}

fn iterate(monkeys: &mut Vec<Monkey>, division: usize, rounds: usize) -> Option<usize> {
    let reduce_factor: usize = monkeys.iter().map(|monkey| monkey.test_divisible).product();
    let mut scores = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len().to_owned() {
            let items_len = monkeys[i].items.len();
            while let Some(item_worry_level) = monkeys[i].items.pop_front() {
                let mut worry = item_worry_level;
                worry = match monkeys[i].operation {
                    Operation::Sum => {
                        monkeys[i].left.unwrap_or(worry) + monkeys[i].right.unwrap_or(worry)
                    }
                    Operation::Multiplication => {
                        monkeys[i].left.unwrap_or(worry) * monkeys[i].right.unwrap_or(worry)
                    }
                };
                worry /= division;
                worry %= reduce_factor;
                let target_monkey = if worry % monkeys[i].test_divisible == 0 {
                    monkeys[i].case_true
                } else {
                    monkeys[i].case_false
                };
                monkeys[target_monkey].items.push_back(worry);
            }
            scores[i] += items_len;
        }
    }
    scores.sort();
    scores.reverse();
    scores.iter().take(2).product::<usize>().into()
}

pub fn part1(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    iterate(&mut monkeys, 3, 20)
}

pub fn part2(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    iterate(&mut monkeys, 1, 10000)
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 11;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(10605));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(50830));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(2713310158));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(14399640002));
    }
}
