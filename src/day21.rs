use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Op {
    Value(isize),
    Plus(String, String),
    Minus(String, String),
    Divided(String, String),
    Multiplied(String, String),
    Unknown,
}

fn parse_input(input: &str) -> HashMap<&str, Op> {
    let monkeys = input
        .lines()
        .map(|line| {
            let (monkey, job_str) = line.split_once(": ").unwrap();
            let job = match job_str.parse::<isize>().ok() {
                Some(number) => Op::Value(number),
                None => {
                    let (left, op, right) =
                        job_str.split(' ').collect_tuple::<(_, _, _)>().unwrap();
                    match op {
                        "+" => Op::Plus(left.to_string(), right.to_string()),
                        "-" => Op::Minus(left.to_string(), right.to_string()),
                        "/" => Op::Divided(left.to_string(), right.to_string()),
                        "*" => Op::Multiplied(left.to_string(), right.to_string()),
                        _ => panic!("Unexpected op {}", op),
                    }
                }
            };
            (monkey, job)
        })
        .collect::<HashMap<_, _>>();
    monkeys
}

fn eval(monkeys: &HashMap<&str, Op>, monkey: &str) -> Option<isize> {
    match monkeys.get(monkey).unwrap() {
        Op::Value(number) => Some(*number),
        Op::Plus(left, right) => {
            if let (Some(left_eval), Some(right_eval)) = (eval(monkeys, left), eval(monkeys, right))
            {
                Some(left_eval + right_eval)
            } else {
                None
            }
        }
        Op::Minus(left, right) => {
            if let (Some(left_eval), Some(right_eval)) = (eval(monkeys, left), eval(monkeys, right))
            {
                Some(left_eval - right_eval)
            } else {
                None
            }
        }
        Op::Divided(left, right) => {
            if let (Some(left_eval), Some(right_eval)) = (eval(monkeys, left), eval(monkeys, right))
            {
                Some(left_eval / right_eval)
            } else {
                None
            }
        }
        Op::Multiplied(left, right) => {
            if let (Some(left_eval), Some(right_eval)) = (eval(monkeys, left), eval(monkeys, right))
            {
                Some(left_eval * right_eval)
            } else {
                None
            }
        }
        Op::Unknown => None,
    }
}

pub fn part1(input: &str) -> Option<isize> {
    let monkeys = parse_input(input);
    eval(&monkeys, "root")
}

fn solve_unknown(monkeys: &HashMap<&str, Op>, monkey: &str, solution: isize) -> isize {
    match monkeys.get(monkey).unwrap() {
        Op::Value(_) => panic!("Unexpected"),
        Op::Plus(left, right) => {
            if let Some(left_eval) = eval(monkeys, left) {
                solve_unknown(monkeys, right, solution - left_eval)
            } else if let Some(right_eval) = eval(monkeys, right) {
                solve_unknown(monkeys, left, solution - right_eval)
            } else {
                panic!("Unexpected")
            }
        }
        Op::Minus(left, right) => {
            if let Some(left_eval) = eval(monkeys, left) {
                solve_unknown(monkeys, right, left_eval - solution)
            } else if let Some(right_eval) = eval(monkeys, right) {
                solve_unknown(monkeys, left, solution + right_eval)
            } else {
                panic!("Unexpected")
            }
        }
        Op::Divided(left, right) => {
            if let Some(left_eval) = eval(monkeys, left) {
                solve_unknown(monkeys, right, left_eval / solution)
            } else if let Some(right_eval) = eval(monkeys, right) {
                solve_unknown(monkeys, left, solution * right_eval)
            } else {
                panic!("Unexpected")
            }
        }
        Op::Multiplied(left, right) => {
            if let Some(left_eval) = eval(monkeys, left) {
                solve_unknown(monkeys, right, solution / left_eval)
            } else if let Some(right_eval) = eval(monkeys, right) {
                solve_unknown(monkeys, left, solution / right_eval)
            } else {
                panic!("Unexpected")
            }
        }
        Op::Unknown => solution,
    }
}

pub fn part2(input: &str) -> Option<isize> {
    let mut monkeys = parse_input(input);
    monkeys.insert("humn", Op::Unknown);

    let (left, right) = match monkeys.get("root").unwrap() {
        Op::Plus(left, right) => (left, right),
        Op::Minus(left, right) => (left, right),
        Op::Divided(left, right) => (left, right),
        Op::Multiplied(left, right) => (left, right),
        _ => panic!("Unexpected"),
    };

    if let Some(left_eval) = eval(&monkeys, left) {
        solve_unknown(&monkeys, right, left_eval).into()
    } else if let Some(right_eval) = eval(&monkeys, right) {
        solve_unknown(&monkeys, left, right_eval).into()
    } else {
        panic!("Unexpected")
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 21;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(152));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(364367103397416));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(301));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(3782852515583));
    }
}
