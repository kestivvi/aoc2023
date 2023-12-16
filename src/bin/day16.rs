use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use timed::timed;

const DAY: u8 = 16;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();
    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

struct LaserGrid {
    grid: Vec<Vec<char>>,
}

impl LaserGrid {
    fn new_from_input(input: &str) -> Self {
        Self {
            grid: input.trim().lines().map(|line| line.trim().chars().collect_vec()).collect_vec(),
        }
    }

    fn calculate_energized_positions_after_shot_from(
        &self,
        start_position: (usize, usize),
        start_direction: (isize, isize),
    ) -> HashSet<((usize, usize), (isize, isize))> {
        let mut visited_positions = HashSet::with_capacity(15000);
        let mut stack = Vec::with_capacity(128);
        stack.push((start_position, start_direction));

        while let Some((current_position, direction)) = stack.pop() {
            if !visited_positions.insert((current_position, direction)) {
                continue;
            }

            let new_projected_position = (
                current_position.0.checked_add_signed(direction.0),
                current_position.1.checked_add_signed(direction.1),
            );

            let new_position = match new_projected_position {
                (Some(new_y), Some(new_x))
                    if new_y < self.grid.len() && new_x < self.grid[new_y].len() =>
                {
                    (new_y, new_x)
                }
                _ => continue,
            };

            match self.grid[new_position.0][new_position.1] {
                '.' => stack.push((new_position, direction)),
                '/' => stack.push((new_position, (-direction.1, -direction.0))),
                '\\' => stack.push((new_position, (direction.1, direction.0))),
                '|' if direction.1 == 0 => stack.push((new_position, direction)),
                '|' if direction.0 == 0 => {
                    stack.push((new_position, (1, 0)));
                    stack.push((new_position, (-1, 0)));
                }
                '-' if direction.1 == 0 => {
                    stack.push((new_position, (0, 1)));
                    stack.push((new_position, (0, -1)));
                }
                '-' if direction.0 == 0 => stack.push((new_position, direction)),
                _ => unreachable!(),
            }
        }

        visited_positions
    }

    fn calculate_number_of_energized_positions(
        &self,
        start_position: (usize, usize),
        start_direction: (isize, isize),
    ) -> usize {
        self.calculate_energized_positions_after_shot_from(start_position, start_direction)
            .iter()
            .map(|(position, _vector)| position)
            .sorted()
            .dedup()
            .count()
    }
}

#[timed]
fn part1(input: &str) -> usize {
    LaserGrid::new_from_input(input).calculate_number_of_energized_positions((0, 0), (0, 1))
}

#[timed]
fn part2(input: &str) -> usize {
    let laser_grid = LaserGrid::new_from_input(input);
    let height = laser_grid.grid.len();
    let width = laser_grid.grid[0].len();

    let vertical_start_configurations =
        (0..height).flat_map(|start_y| [((start_y, 0), (0, 1)), ((start_y, width - 1), (0, -1))]);

    let horizontal_start_configurations =
        (0..width).flat_map(|start_x| [((0, start_x), (1, 0)), ((height - 1, start_x), (-1, 0))]);

    vertical_start_configurations
        .chain(horizontal_start_configurations)
        .par_bridge()
        .map(|(start_position, start_direction)| {
            laser_grid.calculate_number_of_energized_positions(start_position, start_direction)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = read_input(DAY, InputType::Test).unwrap();
        let expected = 46;
        let result = part1(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let input = read_input(DAY, InputType::Real).unwrap();
        let expected = 7927;
        let result = part1(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = read_input(DAY, InputType::Test).unwrap();
        let expected = 51;
        let result = part2(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_real() {
        let input = read_input(DAY, InputType::Real).unwrap();
        let expected = 8246;
        let result = part2(&input);
        assert_eq!(result, expected);
    }
}
