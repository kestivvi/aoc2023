use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use num::Integer;
use timed::timed;

const DAY: u8 = 8;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[timed]
fn part1(input: &str) -> u64 {
    let (instructions_str, nodes_str) = input.split("\r\n\r\n").collect_tuple().unwrap();
    let instructions = instructions_str.trim().chars().collect_vec();
    let nodes = nodes_str
        .trim()
        .lines()
        .map(|line| {
            let (from, to) = line.trim().split("=").map(|v| v.trim()).collect_tuple().unwrap();
            let (to_left, to_right) = to
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split(",")
                .map(|v| v.trim())
                .collect_tuple()
                .unwrap();

            (from, (to_left, to_right))
        })
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.insert(from, to);
            map
        });

    let mut steps = 0;
    let mut current_node = "AAA";

    for instruction in instructions.into_iter().cycle() {
        current_node = if instruction == 'L' {
            nodes.get(current_node).unwrap().0
        } else {
            nodes.get(current_node).unwrap().1
        };

        steps += 1;
        // dbg!(steps);

        if current_node == "ZZZ" {
            break;
        }
    }

    steps
}

#[timed]
fn part2(input: &str) -> u64 {
    let (instructions_str, nodes_str) = input.split("\r\n\r\n").collect_tuple().unwrap();
    let instructions = instructions_str.trim().chars().collect_vec();
    let nodes = nodes_str
        .trim()
        .lines()
        .map(|line| {
            let (from, to) = line.trim().split("=").map(|v| v.trim()).collect_tuple().unwrap();
            let (to_left, to_right) = to
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split(",")
                .map(|v| v.trim())
                .collect_tuple()
                .unwrap();

            (from, (to_left, to_right))
        })
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.insert(from, to);
            map
        });

    nodes
        .iter()
        .filter(|(k, v)| k.ends_with("A"))
        .map(|(k, v)| *k)
        .map(|start| {
            let mut current_node = start;
            let mut steps = 0;
            for instruction in instructions.iter().cycle() {
                current_node = if instruction == &'L' {
                    nodes.get(current_node).unwrap().0
                } else {
                    nodes.get(current_node).unwrap().1
                };

                steps += 1;
                // dbg!(steps);

                if current_node.ends_with("Z") {
                    break;
                }
            }

            steps
        })
        .reduce(|prev, x| prev.lcm(&x))
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
        let expected = 2;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test2() {
        let expected = 6;
        let result = part1(&read_input(DAY, InputType::Other("test2".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 6;
        let result = part2(&read_input(DAY, InputType::Other("test3".to_string())).unwrap());
        assert_eq!(result, expected);
    }
}
