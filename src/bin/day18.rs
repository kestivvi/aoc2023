use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use timed::timed;

const DAY: u8 = 18;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

type Coords = (usize, usize);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point((i128, i128));

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vector(&self) -> Point {
        match self {
            Direction::Up => Point((-1, 0)),
            Direction::Right => Point((0, 1)),
            Direction::Down => Point((1, 0)),
            Direction::Left => Point((0, -1)),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: usize,
    color: String,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (direction_str, amount_str, color_str) =
                line.trim().split(' ').collect_tuple().unwrap();

            Instruction {
                direction: direction_str.into(),
                amount: amount_str.parse().unwrap(),
                color: color_str.to_string(),
            }
        })
        .collect_vec()
}

fn parse_input_part2(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, _, color_str) = line.trim().split(' ').collect_tuple().unwrap();

            let hex = color_str
                .trim()
                .trim_start_matches("(#")
                .trim_end_matches(")")
                .chars()
                .collect_vec();

            let amount_hex_str = (&hex[0..=4]).iter().collect::<String>();
            let direction_char = &hex[5];

            let amount = usize::from_str_radix(&amount_hex_str, 16).unwrap();
            let direction = Direction::from(direction_char.to_digit(10).unwrap() as u8);

            Instruction { direction, amount, color: color_str.to_string() }
        })
        .collect_vec()
}

fn dig_edge(instructions: &[Instruction]) -> HashSet<Point> {
    let mut trench_coords = HashSet::new();
    let mut current_position = Point((0, 0));
    trench_coords.insert(current_position);

    instructions.iter().for_each(|instruction| {
        for i in 0..instruction.amount {
            current_position = current_position + instruction.direction.to_vector();
            trench_coords.insert(current_position);
        }
    });

    trench_coords
}

fn dig_edge_and_calculate_interior(instructions: &[Instruction]) -> u128 {
    let mut area: i128 = 0;
    let mut perimeter: i128 = 0;

    let mut current_position = Point((0, 0));

    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.amount {
            let new_position = current_position + instruction.direction.to_vector();
            area += ((current_position.0 .0) * (new_position.0 .1)) as i128;
            area -= ((new_position.0 .0) * (current_position.0 .1)) as i128;
            perimeter += (current_position.0 .0 - new_position.0 .0).abs() as i128;
            perimeter += (current_position.0 .1 - new_position.0 .1).abs() as i128;

            current_position = new_position;
        }
    });

    (area.abs() / 2 + perimeter / 2 + 1) as u128
}

// fn get_bounds(trench: &HashSet<Point>) -> (isize, isize, isize, isize) {
//     let mut y_lower_bound: isize = 0;
//     let mut y_upper_bound: isize = 0;
//     let mut x_lower_bound: isize = 0;
//     let mut x_upper_bound: isize = 0;

//     for point in trench {
//         let (y, x) = point.0;
//         y_lower_bound = y_lower_bound.min(y);
//         y_upper_bound = y_upper_bound.max(y);
//         x_lower_bound = x_lower_bound.min(x);
//         x_upper_bound = x_upper_bound.max(x);
//     }

//     (y_lower_bound, y_upper_bound, x_lower_bound, x_upper_bound)
// }

// fn count_interior_with_edge(trench: &HashSet<Point>) -> u128 {
//     dbg!("trench size", trench.len());
//     let (y_lower_bound, y_upper_bound, x_lower_bound, x_upper_bound) = get_bounds(trench);
//     dbg!("found bounds");

//     let counter: u128 = (y_lower_bound..=y_upper_bound)
//         .into_par_iter()
//         .map(|y| {
//             let mut counter = 0;
//             let mut inside = false;
//             for x in x_lower_bound..=x_upper_bound {
//                 let current_point = Point((y, x));
//                 let one_below_current_point = Point((y + 1, x));
//                 if trench.contains(&current_point) && trench.contains(&one_below_current_point) {
//                     inside = !inside;
//                 }
//                 if trench.contains(&current_point) {
//                     continue;
//                 }
//                 if inside {
//                     counter += 1;
//                 }
//             }

//             // let progress = (y - y_lower_bound) as f32 / (y_upper_bound - y_lower_bound) as f32;

//             counter
//         })
//         .sum();

//     counter + trench.len() as u128
// }

#[timed]
fn part1(input: &str) -> u128 {
    let instructions = parse_input(input);
    // let edge = dig_edge(&instructions);
    // count_interior_with_edge(&edge)
    dig_edge_and_calculate_interior(&instructions)
}

#[timed]
fn part2(input: &str) -> u128 {
    let instructions = parse_input_part2(input);
    // dbg!("Dig edge");
    // let edge = dig_edge(&instructions);
    // dbg!("Edge dog");
    // count_interior_with_edge(&edge)
    dig_edge_and_calculate_interior(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 62;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let expected = 47139;
        let result = part1(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 952408144115;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn area_test2() {
        fn shoelace_formula(vertices: &[(f64, f64)]) -> f64 {
            let mut area = 0.0;
            let mut perimeter = 0.0;
            let n = vertices.len();

            for i in 0..n {
                let j = (i + 1) % n;
                area += (vertices[i].0) * (vertices[j].1);
                area -= (vertices[j].0) * (vertices[i].1);
                perimeter +=
                    (vertices[i].0 - vertices[j].0).abs() + (vertices[i].1 - vertices[j].1).abs();
            }

            area.abs() / 2.0 + perimeter / 2.0 + 1.0
        }

        // let polygon = vec![(1.0, 1.0), (1.0, 4.0), (5.0, 4.0), (5.0, 1.0)];
        let vertices = vec![(1.0, 1.0), (5.0, 4.0), (1.0, 4.0), (5.0, 1.0)];
        // How to order vertices clockwise?

        let area = shoelace_formula(&vertices);
        println!("{}", area);
    }
}
