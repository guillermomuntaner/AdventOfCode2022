#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Hand {
    Paper,
    Rock,
    Scissors,
}

impl Hand {
    fn parse_opponent(char: &char) -> Hand {
        match char {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("Unexpected direction"),
        }
    }

    fn parse_mine(char: &char) -> Hand {
        match char {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Unexpected direction"),
        }
    }

    fn shape_score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    fn outcome_score_against(&self, opponent: &Hand) -> usize {
        match (self, opponent) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 0,
            (Hand::Rock, Hand::Scissors) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scissors) => 0,
            (Hand::Scissors, Hand::Rock) => 0,
            (Hand::Scissors, Hand::Paper) => 6,
            (Hand::Scissors, Hand::Scissors) => 3,
        }
    }
}

pub fn part1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| {
            let mut round = line.chars();
            let opponent = Hand::parse_opponent(&round.next().unwrap());
            let mine = Hand::parse_mine(&round.nth(1).unwrap());
            (opponent, mine)
        })
        .fold(0, |score, (opponent, mine)| {
            score + mine.shape_score() + mine.outcome_score_against(&opponent)
        })
        .into()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Result {
    Lose,
    Draw,
    Win,
}

impl Result {
    fn parse(char: &char) -> Result {
        match char {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Unexpected result"),
        }
    }
}

impl Hand {
    fn for_result_against(result: &Result, opponent: &Hand) -> Hand {
        match opponent {
            Hand::Rock => match result {
                Result::Lose => Hand::Scissors,
                Result::Draw => Hand::Rock,
                Result::Win => Hand::Paper,
            },
            Hand::Paper => match result {
                Result::Lose => Hand::Rock,
                Result::Draw => Hand::Paper,
                Result::Win => Hand::Scissors,
            },
            Hand::Scissors => match result {
                Result::Lose => Hand::Paper,
                Result::Draw => Hand::Scissors,
                Result::Win => Hand::Rock,
            },
        }
    }
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| {
            let mut round = line.chars();
            let opponent = Hand::parse_opponent(&round.next().unwrap());
            let result = Result::parse(&round.nth(1).unwrap());
            let mine = Hand::for_result_against(&result, &opponent);
            (opponent, mine)
        })
        .fold(0, |score, (opponent, mine)| {
            score + mine.shape_score() + mine.outcome_score_against(&opponent)
        })
        .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 2;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(15));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(10310));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(12));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(14859));
    }
}
