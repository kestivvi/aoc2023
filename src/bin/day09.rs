use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 9;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Debug)]
struct NumberSequence {
    numbers: Vec<i64>,
    differential: Option<Box<NumberSequence>>,
}

impl NumberSequence {
    fn calculate_differentials_all_the_way_down(&mut self) {
        let differentials = self.numbers.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

        if differentials.iter().all(|v| *v == 0) {
            return;
        }

        let differential = NumberSequence { numbers: differentials, differential: None };
        self.differential = Some(Box::new(differential));
        self.differential.as_mut().unwrap().calculate_differentials_all_the_way_down()
    }

    fn calculate_next(&mut self) {
        if let None = self.differential {
            let next_value = self.numbers.last().unwrap();
            self.numbers.push(*next_value);
            return;
        }

        let differential = self.differential.as_mut().unwrap();
        differential.calculate_next();

        let next_value = self.numbers.last().unwrap() + differential.numbers.last().unwrap();
        self.numbers.push(next_value);
    }

    fn calculate_previous(&mut self) {
        if let None = self.differential {
            let next_value = self.numbers.first().unwrap();
            self.numbers.insert(0, *next_value);
            return;
        }

        let differential = self.differential.as_mut().unwrap();
        differential.calculate_previous();

        let next_value = self.numbers.first().unwrap() - differential.numbers.first().unwrap();
        self.numbers.insert(0, next_value);
    }
}

#[timed]
fn part1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().flat_map(|num| num.parse::<i64>()).collect_vec())
        .map(|numbers| {
            let mut number_sequence = NumberSequence { numbers, differential: None };
            number_sequence.calculate_differentials_all_the_way_down();
            number_sequence.calculate_next();
            number_sequence.numbers.last().unwrap().clone()
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().flat_map(|num| num.parse::<i64>()).collect_vec())
        .map(|numbers| {
            let mut number_sequence = NumberSequence { numbers, differential: None };
            number_sequence.calculate_differentials_all_the_way_down();
            number_sequence.calculate_previous();
            number_sequence.numbers.first().unwrap().clone()
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
        let expected = 114;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 2;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
