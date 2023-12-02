use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::convert::From;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Draw {
    Blue(u64),
    Green(u64),
    Red(u64),
}

impl From<&str> for Draw {
    fn from(raw: &str) -> Self {
        let chunks = raw.trim().split(" ").collect::<Vec<&str>>();
        let amount = chunks[0].parse::<u64>().unwrap();
        match chunks[1] {
            "red" => Self::Red(amount),
            "green" => Self::Green(amount),
            "blue" => Self::Blue(amount),
            _ => unreachable!("unrecognized amount"),
        }
    }
}

impl Draw {
    fn is_valid(&self, limits: (u64, u64, u64)) -> bool {
        match self {
            Self::Red(x) => *x <= limits.0,
            Self::Green(x) => *x <= limits.1,
            Self::Blue(x) => *x <= limits.2,
        }
    }
}

pub type Game = (u64, Vec<Vec<Draw>>);

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Game> {
    let re_number = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let chunks = line.split(":").collect::<Vec<&str>>();
            let id = re_number.find(chunks[0]).unwrap().as_str();
            let draws: Vec<Vec<Draw>> = chunks[1]
                .split(";")
                .map(|x| x.split(", ").map(|y| Draw::from(y)).collect())
                .collect();
            (id.parse::<u64>().unwrap(), draws)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u64 {
    const LIMITS: (u64, u64, u64) = (12, 13, 14);

    input
        .into_iter()
        .filter(|game| {
            let sets = game.1.clone();
            sets.into_iter()
                .all(|set| set.into_iter().all(|draw| draw.is_valid(LIMITS)))
        })
        .map(|(id, _)| id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_from_str() {
        assert_eq!(Draw::from("3 blue"), Draw::Blue(3));
        assert_eq!(Draw::from("4 red"), Draw::Red(4));
        assert_eq!(Draw::from("12 green"), Draw::Green(12));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![(
                1,
                vec![
                    vec![Draw::Blue(3), Draw::Red(4)],
                    vec![Draw::Red(1), Draw::Green(2), Draw::Blue(6)],
                    vec![Draw::Green(2)]
                ]
            )],
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part1(&parse_input(input)), 8);
    }
}
