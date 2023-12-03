use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 3;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

struct Schematic(Vec<Vec<char>>);

#[derive(Debug)]
enum Char {
    Symbol(char),
    Digit(char),
    None,
}

impl Schematic {
    fn get_char(&self, y: usize, x: usize) -> Char {
        match self.0.get(y) {
            Some(line) => match line.get(x) {
                Some(c) => {
                    if "!@#$%^&*()_+-=<>?:\"{}[];',/".contains(*c) {
                        Char::Symbol(*c)
                    } else if c.is_digit(10) {
                        Char::Digit(*c)
                    } else if *c == '.' {
                        Char::None
                    } else {
                        unreachable!("Not expected char {}", c);
                        // Char::None
                    }
                }
                None => Char::None,
            },
            None => Char::None,
        }
    }

    fn get_part_number_at(
        &self,
        y: usize,
        x: usize,
        read_positions: &mut HashSet<(usize, usize)>,
    ) -> Option<u32> {
        match self.get_char(y, x) {
            Char::Symbol(_) => return None,
            Char::Digit(_) => (),
            Char::None => return None,
        }

        let mut tmp_x: i32 = x as i32 - 1;

        while let Char::Digit(_) = self.get_char(y, tmp_x as usize) {
            tmp_x -= 1;
            if tmp_x < 0 {
                break;
            }
        }

        let mut digits = Vec::<char>::new();
        tmp_x += 1;
        while let Char::Digit(digit) = self.get_char(y, tmp_x as usize) {
            read_positions.insert((y, tmp_x as usize));
            digits.push(digit);
            tmp_x += 1;
        }
        Some(digits.iter().collect::<String>().parse().unwrap())
    }

    fn read_valid_parts(&self) -> Vec<u32> {
        let mut read_positions = HashSet::<(usize, usize)>::new();
        let mut valid_parts = Vec::<u32>::new();

        self.0.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, char)| {
                let current_char = self.get_char(y, x);
                match current_char {
                    Char::Symbol(_) => {
                        let positions = [
                            (y - 1, x),
                            (y - 1, x + 1),
                            (y - 1, x - 1),
                            (y, x - 1),
                            (y, x + 1),
                            (y + 1, x - 1),
                            (y + 1, x),
                            (y + 1, x + 1),
                        ];
                        for (y, x) in positions {
                            if read_positions.contains(&(y, x)) {
                                continue;
                            }
                            let num = self.get_part_number_at(y, x, &mut read_positions);
                            if let Some(num) = num {
                                valid_parts.push(num);
                            }
                        }
                    }
                    _ => (),
                }
            })
        });

        return valid_parts;
    }

    fn read_valid_gears(&self) -> Vec<(u32, u32)> {
        let mut read_positions = HashSet::<(usize, usize)>::new();
        let mut gear_pairs = Vec::<(u32, u32)>::new();

        self.0.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, char)| {
                let current_char = self.get_char(y, x);
                match current_char {
                    Char::Symbol(c) if c == '*' => {
                        let mut current_gears = Vec::<u32>::new();
                        let positions = [
                            (y - 1, x),
                            (y - 1, x + 1),
                            (y - 1, x - 1),
                            (y, x - 1),
                            (y, x + 1),
                            (y + 1, x - 1),
                            (y + 1, x),
                            (y + 1, x + 1),
                        ];
                        for (y, x) in positions {
                            if read_positions.contains(&(y, x)) {
                                continue;
                            }
                            let num = self.get_part_number_at(y, x, &mut read_positions);
                            if let Some(num) = num {
                                current_gears.push(num);
                            }
                        }
                        if current_gears.len() == 2 {
                            gear_pairs.push((
                                current_gears.get(0).unwrap().clone(),
                                current_gears.get(1).unwrap().clone(),
                            ))
                        }
                    }
                    _ => (),
                }
            })
        });

        return gear_pairs;
    }
}

#[timed]
fn part1(input: &str) -> u32 {
    let schematic = {
        let schematic = input
            .lines()
            .map(|line| line.trim().chars().collect_vec())
            .collect_vec();
        Schematic(schematic)
    };

    schematic.read_valid_parts().iter().sum()
}

#[timed]
fn part2(input: &str) -> u32 {
    let schematic = {
        let schematic = input
            .lines()
            .map(|line| line.trim().chars().collect_vec())
            .collect_vec();
        Schematic(schematic)
    };

    schematic
        .read_valid_gears()
        .iter()
        .map(|(a, b)| a * b)
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
        let expected = 4361;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 467835;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
