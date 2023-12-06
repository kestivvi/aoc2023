use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use timed::timed;

const DAY: u8 = 6;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn calculate_distance(holding_time: u64, end_time: u64) -> u64 {
    if holding_time >= end_time {
        return 0;
    }

    let mut distance = 0;
    let mut remaining_time = end_time;
    let mut speed = 0;

    for _ in 0..holding_time {
        speed += 1;
        remaining_time -= 1;
    }

    while remaining_time > 0 {
        distance += speed;
        remaining_time -= 1;
    }

    dbg!(distance)
}

fn calculate_num_of_ways_to_win(end_time: u64, record_distance: u64) -> u64 {
    dbg!((end_time, record_distance));
    let result = (1..end_time)
        // .into_par_iter()
        .map(|holding_time| {
            (calculate_distance(holding_time, end_time) > record_distance)
                .then_some(1)
                .unwrap_or(0)
        })
        .sum();
    result
}

fn calculate_num_of_ways_to_win2(end_time: u64, record_distance: u64) -> u64 {
    dbg!((end_time, record_distance));
    let result = (1..end_time)
        // .into_par_iter()
        .map(|holding_time| {
            (calculate_distance(holding_time, end_time) > record_distance)
                .then_some(1)
                .unwrap_or(0)
        })
        .sum();
    result
}

#[timed]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .flat_map(|num| num.parse::<u64>())
                .collect_vec()
        })
        .tuples()
        .map(|(end_time, record_distance)| {
            end_time
                .iter()
                .zip(record_distance)
                .map(|(end_time, record_distance)| {
                    calculate_num_of_ways_to_win(*end_time, record_distance)
                })
                .product()
        })
        .next()
        .unwrap()
}

#[timed]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(":")
                .last()
                .unwrap()
                .trim()
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .tuples()
        .map(|(end_time, record_distance)| calculate_num_of_ways_to_win(end_time, record_distance))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 288;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 71503;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
