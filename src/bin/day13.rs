use std::fmt::Display;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 13;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Clone, PartialEq, Debug)]
enum Tile {
    Ash,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => f.write_str("."),
            Tile::Rock => f.write_str("#"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Ash),
            '#' => Ok(Tile::Rock),
            _ => unreachable!("WTF?"),
        }
    }
}

struct Block(Vec<Vec<Tile>>);

impl Block {
    fn from_input(input: &str) -> Vec<Self> {
        input
            .trim()
            .split("\r\n\r\n")
            .map(|block| {
                let parsed_block = block
                    .trim()
                    .lines()
                    .map(|line| line.chars().flat_map(Tile::try_from).collect_vec())
                    .collect_vec();
                Self(parsed_block)
            })
            .collect_vec()
    }

    fn get_height(&self) -> usize {
        self.0.len()
    }

    fn get_width(&self) -> usize {
        self.0[0].len()
    }

    fn mirror_line_index(&self) -> Option<usize> {
        // self.print();
        let mut best_line = None;
        let mut most_matches = None;

        for line in 0..self.get_height() {
            let mut current_matches = 0;
            let mut offset = 0;
            loop {
                let line_up_index = line.checked_sub(offset);
                let line_up = if line_up_index.is_some() {
                    self.0.get(line_up_index.unwrap())
                } else {
                    None
                };
                let line_down = self.0.get(line + offset + 1);
                if let (Some(line_up), Some(line_down)) = (line_up, line_down) {
                    if line_up != line_down {
                        break;
                    }
                    current_matches += 1;
                } else {
                    if offset > 0
                        && (most_matches.is_none()
                            || most_matches
                                .is_some_and(|most_matches| current_matches > most_matches))
                    {
                        most_matches = Some(current_matches);
                        best_line = Some(line);
                    }
                    break;
                }
                offset += 1;
            }
        }

        // dbg!(&best_line);
        best_line
    }

    fn horizontal_mirror_line_index(&self) -> Option<usize> {
        self.mirror_line_index()
    }

    fn transposed(&self) -> Self {
        let transposed = (0..self.get_width())
            .map(|x| {
                (0..self.get_height())
                    .map(move |y| self.0.get(y).unwrap().get(x).unwrap().clone())
                    .collect_vec()
            })
            .collect_vec();

        Self(transposed)
    }

    fn vertical_mirror_line_index(&self) -> Option<usize> {
        self.transposed().mirror_line_index()
    }

    fn print(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|tile| print!("{}", tile));
            println!();
        });
        println!();
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let blocks = Block::from_input(input);

    blocks
        .iter()
        .map(|block| {
            let horizontal = block.horizontal_mirror_line_index();
            let vertical = block.vertical_mirror_line_index();

            if let Some(horizontal) = horizontal {
                100 * (horizontal + 1)
            } else if let Some(vertical) = vertical {
                vertical + 1
            } else {
                0
            }
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 405;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 0;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
