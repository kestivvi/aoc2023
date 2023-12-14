use aoc2023::{read_input, InputType};
use itertools::Itertools;
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

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn from_input(input: &str) -> Self {
        let parsed = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(Tile::from).collect_vec())
            .collect_vec();
        Self(parsed)
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
                for y in 0..self.0.len() {
                    for x in 0..self.0[0].len() {
                        if self.0[y][x] == Tile::RoundRock {
                            let (new_y, new_x) =
                                self.get_new_position_in_direction(y, x, direction);
                            self.0[y][x] = Tile::Space;
                            self.0[new_y][new_x] = Tile::RoundRock;
                            // print_grid(&grid);
                        }
                    }
                }
            }
            Direction::West => {
                for x in 0..self.0[0].len() {
                    for y in 0..self.0.len() {
                        if self.0[y][x] == Tile::RoundRock {
                            let (new_y, new_x) =
                                self.get_new_position_in_direction(y, x, direction);
                            self.0[y][x] = Tile::Space;
                            self.0[new_y][new_x] = Tile::RoundRock;
                            // print_grid(&grid);
                        }
                    }
                }
            }
            Direction::South => {
                for y in (self.0.len() - 1)..=0 {
                    for x in 0..self.0[0].len() {
                        if self.0[y][x] == Tile::RoundRock {
                            let (new_y, new_x) =
                                self.get_new_position_in_direction(y, x, direction);
                            self.0[y][x] = Tile::Space;
                            self.0[new_y][new_x] = Tile::RoundRock;
                            // print_grid(&grid);
                        }
                    }
                }
            }
            Direction::East => {
                for x in (self.0[0].len() - 1)..=0 {
                    for y in 0..self.0.len() {
                        if self.0[y][x] == Tile::RoundRock {
                            let (new_y, new_x) =
                                self.get_new_position_in_direction(y, x, direction);
                            self.0[y][x] = Tile::Space;
                            self.0[new_y][new_x] = Tile::RoundRock;
                            // print_grid(&grid);
                        }
                    }
                }
            }
        }

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
}

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{c}"));
        println!();
    });
    println!();
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
    // print_grid(&grid);
    // print_grid(&grid_tilted_north);

    let upper_bound = 1_000_000_000;
    for i in 0..upper_bound {
        if i % 1_000_000 == 0 {
            println!("{}", i as f32 / upper_bound as f32);
        }
        grid.move_rocks_in_direction(Direction::North)
            .move_rocks_in_direction(Direction::West)
            .move_rocks_in_direction(Direction::South)
            .move_rocks_in_direction(Direction::East);
    }

    grid.calculate_north_load()
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
    fn part2_test() {
        let expected = 64;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
