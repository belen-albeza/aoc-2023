use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    lazy_static! {
        static ref NOT_NUMBERS: Regex = Regex::new(r"\D").unwrap();
    }

    let numbers: Vec<u64> = input
        .into_iter()
        .map(|x| {
            let digits: Vec<char> = NOT_NUMBERS.replace_all(x, "").to_string().chars().collect();
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u64>()
                .unwrap();
            number
        })
        .collect();

    numbers.into_iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    input
        .into_iter()
        .map(|x| parse_digits(x))
        .map(|x| {
            let num = format!("{}{}", x.0, x.1);
            num.parse::<u64>().unwrap()
        })
        .sum()
}

fn parse_digits(input: &str) -> (u64, u64) {
    lazy_static! {
        static ref DIGITS_LEFT_RE: Regex =
            Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        static ref DIGITS_RIGHT_RE: Regex =
            Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    }

    let first = DIGITS_LEFT_RE.find(input).unwrap().as_str();
    let last = DIGITS_RIGHT_RE
        .find(&input.chars().rev().collect::<String>())
        .unwrap()
        .as_str()
        .chars()
        .rev()
        .collect::<String>();

    let digits: Vec<u64> = vec![first, last.clone().as_str()]
        .into_iter()
        .map(|x| match x {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => x.parse::<u64>().unwrap(),
        })
        .collect();
    (digits[0], digits[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_digits() {
        assert_eq!(parse_digits("two1nine"), (2, 9));
        assert_eq!(parse_digits("eighttwothree"), (8, 3));
        assert_eq!(parse_digits("abcone2threexyz"), (1, 3));
        assert_eq!(parse_digits("82rbnsg48twoseven"), (8, 7));
        assert_eq!(parse_digits("eighthree"), (8, 3));
    }

    #[test]
    fn test_solve_part2() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve_part2(&parse_input(input)), 281);
    }
}
