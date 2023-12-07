use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 6;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn calculate_num_of_ways_to_win(end_time: u64, record_distance: u64) -> u64 {
    let square_root_delta = ((end_time.pow(2) - 4 * record_distance) as f32).sqrt();

    let left = ((-(end_time as f32) + square_root_delta) / -2_f32).ceil() as u64;
    let right = ((-(end_time as f32) - square_root_delta) / -2_f32).floor() as u64;

    let result = (right).abs_diff(left) + 1;

    if square_root_delta.fract() == 0.0 {
        result - 2
    } else {
        result
    }
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
