use std::fmt::Display;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use timed::timed;

const DAY: u8 = 14;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq)]
enum Tile {
    Space,
    RoundRock,
    SquareRock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Space,
            'O' => Tile::RoundRock,
            '#' => Tile::SquareRock,
            _ => unreachable!("WTF?"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => f.write_str("."),
            Tile::RoundRock => f.write_str("O"),
            Tile::SquareRock => f.write_str("#"),
        }
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn from_input(input: &str) -> Self {
        let parsed = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(Tile::from).collect_vec())
            .collect_vec();
        let grid = Self(parsed);
        // grid.print_grid();
        grid
    }

    fn get_new_position_in_direction(
        &self,
        y: usize,
        x: usize,
        direction: Direction,
    ) -> (usize, usize) {
        let mut new_y = y;
        let mut new_x = x;

        match direction {
            Direction::North => {
                'outer: for temp_y in (0..y).rev() {
                    for temp_x in x..=x {
                        if self.0[temp_y][temp_x] == Tile::Space {
                            new_y = temp_y;
                            new_x = temp_x;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
            Direction::West => {
                'outer: for temp_x in (0..x).rev() {
                    for temp_y in y..=y {
                        if self.0[temp_y][temp_x] == Tile::Space {
                            new_y = temp_y;
                            new_x = temp_x;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
            Direction::South => {
                'outer: for temp_y in (y + 1)..self.0.len() {
                    for temp_x in x..=x {
                        if self.0[temp_y][temp_x] == Tile::Space {
                            new_y = temp_y;
                            new_x = temp_x;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
            Direction::East => {
                'outer: for temp_x in (x + 1)..self.0[y].len() {
                    for temp_y in y..=y {
                        if self.0[temp_y][temp_x] == Tile::Space {
                            new_y = temp_y;
                            new_x = temp_x;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
        }

        // dbg!((y_iter.clone().collect_vec(), x_iter.clone().collect_vec()));

        (new_y, new_x)
    }
    fn move_rocks_in_direction(&mut self, direction: Direction) -> &mut Self {
        match direction {
            Direction::North => {
                for x in 0..self.0[0].len() {
                    let mut last_space = None;
                    for y in 0..self.0.len() {
                        match self.0[y][x] {
                            Tile::Space => {
                                let beginning = y == 0;
                                let there_is_previous = y.checked_sub(1).is_some();
                                let previous_was_rock = there_is_previous
                                    && (self.0[y - 1][x] == Tile::SquareRock
                                        || self.0[y - 1][x] == Tile::RoundRock);
                                if beginning || previous_was_rock {
                                    last_space = Some(y)
                                }
                            }
                            Tile::RoundRock => {
                                if let Some(new_y) = last_space {
                                    self.0[y][x] = Tile::Space;
                                    self.0[new_y][x] = Tile::RoundRock;
                                    last_space = Some(new_y + 1);
                                }
                            }
                            Tile::SquareRock => {
                                last_space = None;
                            }
                        }
                    }
                }
            }

            Direction::West => {
                for y in 0..self.0.len() {
                    let mut last_space = None;
                    for x in 0..self.0[0].len() {
                        match self.0[y][x] {
                            Tile::Space => {
                                let beginning = x == 0;
                                let there_is_previous = x.checked_sub(1).is_some();
                                let previous_was_rock = there_is_previous
                                    && (self.0[y][x - 1] == Tile::SquareRock
                                        || self.0[y][x - 1] == Tile::RoundRock);
                                if beginning || previous_was_rock {
                                    last_space = Some(x)
                                }
                            }
                            Tile::RoundRock => {
                                if let Some(new_x) = last_space {
                                    self.0[y][x] = Tile::Space;
                                    self.0[y][new_x] = Tile::RoundRock;
                                    last_space = Some(new_x + 1);
                                }
                            }
                            Tile::SquareRock => {
                                last_space = None;
                            }
                        }
                    }
                }
            }

            Direction::South => {
                for x in 0..self.0[0].len() {
                    let mut last_space = None;
                    // for y in (self.0.len() - 1)..=0 {
                    for y in (0..self.0.len()).rev() {
                        match self.0[y][x] {
                            Tile::Space => {
                                let beginning = y == self.0.len() - 1;
                                let there_is_previous = y + 1 < self.0.len();
                                let previous_was_rock = there_is_previous
                                    && (self.0[y + 1][x] == Tile::SquareRock
                                        || self.0[y + 1][x] == Tile::RoundRock);
                                if beginning || previous_was_rock {
                                    last_space = Some(y)
                                }
                            }
                            Tile::RoundRock => {
                                if let Some(new_y) = last_space {
                                    self.0[y][x] = Tile::Space;
                                    self.0[new_y][x] = Tile::RoundRock;
                                    last_space = Some(new_y - 1);
                                }
                            }
                            Tile::SquareRock => {
                                last_space = None;
                            }
                        }
                    }
                }
            }
            Direction::East => {
                for y in 0..self.0.len() {
                    let mut last_space = None;
                    // for x in (self.0[0].len() - 1)..=0 {
                    for x in (0..self.0[0].len()).rev() {
                        match self.0[y][x] {
                            Tile::Space => {
                                let beginning = x == self.0[0].len() - 1;
                                let there_is_previous = x + 1 < self.0[0].len();
                                let previous_was_rock = there_is_previous
                                    && (self.0[y][x + 1] == Tile::SquareRock
                                        || self.0[y][x + 1] == Tile::RoundRock);
                                if beginning || previous_was_rock {
                                    last_space = Some(x)
                                }
                            }
                            Tile::RoundRock => {
                                if let Some(new_x) = last_space {
                                    self.0[y][x] = Tile::Space;
                                    self.0[y][new_x] = Tile::RoundRock;
                                    last_space = Some(new_x - 1);
                                }
                            }
                            Tile::SquareRock => {
                                last_space = None;
                            }
                        }
                    }
                }
            }
        }
        // self.print_grid();

        self
    }

    fn calculate_north_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                if self.0[y][x] == Tile::RoundRock {
                    load += self.0.len() - y;
                }
            }
        }

        load
    }
    fn print_grid(&self) {
        self.0.iter().for_each(|line| {
            line.iter().for_each(|c| print!("{c}"));
            println!();
        });
        println!();
    }
}

#[timed]
fn part1(input: &str) -> usize {
    Grid::from_input(input)
        .move_rocks_in_direction(Direction::North)
        .calculate_north_load()
}

#[timed]
fn part2(input: &str) -> usize {
    let mut grid = Grid::from_input(input);
    grid.print_grid();

    let loads = (0..1000)
        .map(|_| {
            grid.move_rocks_in_direction(Direction::North);
            grid.move_rocks_in_direction(Direction::West);
            grid.move_rocks_in_direction(Direction::South);
            grid.move_rocks_in_direction(Direction::East);
            // println!("{} {}", i + 1, grid.calculate_north_load());
            grid.calculate_north_load()
        })
        .collect_vec();

    let last = loads.last().unwrap();
    let mut previous_index = 0;

    for i in (0..(loads.len() - 1)).rev() {
        if loads[i] == *last {
            previous_index = i;
            break;
        }
    }
    dbg!(previous_index);

    let modulo = dbg!(1000 - previous_index - 1);
    let answers = &loads[999 - modulo..999];
    dbg!(answers);
    answers[(1_000_000_000) % modulo + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 136;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let expected = 109661;
        let result = part1(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 64;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_real() {
        let expected = 90176;
        let result = part2(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }
}
