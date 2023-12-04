use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 4;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[timed]
fn part1(input: &str) -> u32 {
    let convert_to_numbers = |input_str: &str| {
        input_str
            .split_whitespace()
            .flat_map(|s| s.trim().parse::<u32>())
            .collect::<Vec<_>>()
    };

    input
        .lines()
        .map(|line| {
            let (_, numbers_part) = line.trim().split(":").collect_tuple().unwrap();
            let (winning_numbers_part, our_numbers_part) =
                numbers_part.trim().split("|").collect_tuple().unwrap();

            let winning_numbers = convert_to_numbers(winning_numbers_part)
                .iter()
                .copied()
                .collect::<HashSet<u32>>();
            let our_numbers = convert_to_numbers(our_numbers_part)
                .iter()
                .copied()
                .collect::<HashSet<u32>>();

            our_numbers
                .intersection(&winning_numbers)
                .collect_vec()
                .len() as u32
        })
        .map(|count| if count == 0 { 0 } else { 2_u32.pow(count - 1) })
        .sum()
}

#[derive(Debug)]
struct Card {
    id: u32,
    instances: u32,
    winning: HashSet<u32>,
    our: HashSet<u32>,
}

#[timed]
fn part2(input: &str) -> u32 {
    let convert_to_numbers = |input_str: &str| {
        input_str
            .split_whitespace()
            .flat_map(|s| s.trim().parse::<u32>())
            .collect::<Vec<_>>()
    };

    let mut cards = input
        .lines()
        .map(|line| {
            let (id_part, numbers_part) = line.trim().split(":").collect_tuple().unwrap();
            let id = id_part.split(" ").last().unwrap().parse::<u32>().unwrap();

            let (winning_numbers_part, our_numbers_part) =
                numbers_part.trim().split("|").collect_tuple().unwrap();

            let winning_numbers = convert_to_numbers(winning_numbers_part)
                .iter()
                .copied()
                .collect::<HashSet<u32>>();
            let our_numbers = convert_to_numbers(our_numbers_part)
                .iter()
                .copied()
                .collect::<HashSet<u32>>();

            Card {
                id,
                instances: 1,
                winning: winning_numbers,
                our: our_numbers,
            }
        })
        .collect_vec();

    for j in 0..cards.len() {
        let card = &cards[j];
        let count_of_winning = card.our.intersection(&card.winning).collect_vec().len() as u32;
        for _ in 0..card.instances {
            if count_of_winning != 0 {
                for i in 1..=count_of_winning {
                    // let x = cards.get_mut((card.id - 1 + i) as usize).unwrap();
                    let index = j + i as usize;
                    let x = &mut cards[index];
                    x.instances += 1;
                    // println!("add 1 to {}, it has now {}", index + 1, x.instances);
                }
            }
        }
    }
    // dbg!(&cards);

    cards.iter().map(|card| card.instances).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 13;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 30;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
