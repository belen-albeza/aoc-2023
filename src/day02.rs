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

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    id: u64,
    sets: Vec<Vec<Draw>>,
}

impl Game {
    fn min_possible_set(&self) -> (u64, u64, u64) {
        let red = self
            .sets
            .iter()
            .map(|set| {
                set.into_iter()
                    .map(|x| match x {
                        Draw::Red(n) => *n,
                        _ => 0,
                    })
                    .sum()
            })
            .max()
            .unwrap();
        let green = self
            .sets
            .iter()
            .map(|set| {
                set.into_iter()
                    .map(|x| match x {
                        Draw::Green(n) => *n,
                        _ => 0,
                    })
                    .sum()
            })
            .max()
            .unwrap();
        let blue = self
            .sets
            .iter()
            .map(|set| {
                set.into_iter()
                    .map(|x| match x {
                        Draw::Blue(n) => *n,
                        _ => 0,
                    })
                    .sum()
            })
            .max()
            .unwrap();

        (red, green, blue)
    }
}

fn power_for_set(set: (u64, u64, u64)) -> u64 {
    set.0 * set.1 * set.2
}

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
            Game {
                id: id.parse::<u64>().unwrap(),
                sets: draws,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u64 {
    const LIMITS: (u64, u64, u64) = (12, 13, 14);

    input
        .into_iter()
        .filter(|game| {
            game.sets
                .clone()
                .into_iter()
                .all(|set| set.into_iter().all(|draw| draw.is_valid(LIMITS)))
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u64 {
    input
        .into_iter()
        .map(|x| x.min_possible_set())
        .map(|set| power_for_set(set))
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
            vec![Game {
                id: 1,
                sets: vec![
                    vec![Draw::Blue(3), Draw::Red(4)],
                    vec![Draw::Red(1), Draw::Green(2), Draw::Blue(6)],
                    vec![Draw::Green(2)]
                ],
            }]
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

    #[test]
    fn test_min_possible_set() {
        let game = Game {
            id: 1,
            sets: vec![
                vec![Draw::Blue(3), Draw::Red(4)],
                vec![Draw::Red(1), Draw::Green(2), Draw::Blue(6)],
                vec![Draw::Green(2)],
            ],
        };
        assert_eq!(game.min_possible_set(), (4, 2, 6));
    }

    #[test]
    fn test_solve_part2() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve_part2(&parse_input(input)), 2286);
    }
}
