use std::{collections::HashMap, str::FromStr};

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 2;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (game_part, sets_part) = line.split(":").collect_tuple().unwrap();
        let id = game_part.split(" ").last().unwrap().parse().unwrap();
        let sets = sets_part
            .split(";")
            .map(|set_str| {
                let sets_map: HashMap<&str, u32> = set_str
                    .split(",")
                    .map(|color| {
                        let (num, name) = color.trim().split(" ").collect_tuple().unwrap();
                        (name, num.parse().unwrap())
                    })
                    .collect();
                Set {
                    red: sets_map.get("red").copied().unwrap_or_default(),
                    green: sets_map.get("green").copied().unwrap_or_default(),
                    blue: sets_map.get("blue").copied().unwrap_or_default(),
                }
            })
            .collect_vec();

        Ok(Game { id, sets })
    }
}

#[timed]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| line.parse::<Game>())
        .map(|game| {
            let invalid = game
                .sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14);
            if invalid {
                0
            } else {
                game.id
            }
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| line.parse::<Game>())
        .map(|game| {
            let maximums = game.sets.iter().fold(Set::default(), |maximums, set| Set {
                red: maximums.red.max(set.red),
                green: maximums.green.max(set.green),
                blue: maximums.blue.max(set.blue),
            });
            maximums.red * maximums.green * maximums.blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 8;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 2286;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
