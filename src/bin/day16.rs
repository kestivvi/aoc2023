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

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new_from_input(input: &str) -> Self {
        let grid = input.trim().lines().map(|line| line.trim().chars().collect_vec()).collect_vec();
        Self { grid }
    }

    fn shot_from(
        &self,
        position: (usize, usize),
        vector: (isize, isize),
    ) -> HashSet<((usize, usize), (isize, isize))> {
        let mut energized = HashSet::with_capacity(15000);

        fn dfs(
            position: (usize, usize),
            vector: (isize, isize),
            grid: &Vec<Vec<char>>,
            energized: &mut HashSet<((usize, usize), (isize, isize))>,
        ) {
            if energized.contains(&(position, vector)) {
                return;
            }
            energized.insert((position, vector));

            let new_projected_position =
                (position.0.checked_add_signed(vector.0), position.1.checked_add_signed(vector.1));

            let new_position = match new_projected_position {
                (Some(new_y), Some(new_x)) if new_y < grid.len() && new_x < grid[new_y].len() => {
                    (new_y, new_x)
                }
                _ => return,
            };

            match grid[new_position.0][new_position.1] {
                '.' => dfs(new_position, vector, grid, energized),
                '/' => dfs(new_position, (-vector.1, -vector.0), grid, energized),
                '\\' => dfs(new_position, (vector.1, vector.0), grid, energized),
                '|' if vector.1 == 0 => dfs(new_position, vector, grid, energized),
                '|' if vector.0 == 0 => {
                    dfs(new_position, (1, 0), grid, energized);
                    dfs(new_position, (-1, 0), grid, energized);
                }
                '-' if vector.1 == 0 => {
                    dfs(new_position, (0, 1), grid, energized);
                    dfs(new_position, (0, -1), grid, energized);
                }
                '-' if vector.0 == 0 => dfs(new_position, vector, grid, energized),
                _ => unreachable!(),
            }
        }

        dfs(position, vector, &self.grid, &mut energized);
        energized
    }

    fn shoot_from_number_of_energized(
        &self,
        position: (usize, usize),
        vector: (isize, isize),
    ) -> usize {
        self.shot_from(position, vector)
            .iter()
            .map(|(position, _vector)| position)
            .sorted()
            .dedup()
            .count()
    }
}

#[timed]
fn part1(input: &str) -> usize {
    Grid::new_from_input(input).shoot_from_number_of_energized((0, 0), (0, 1))
}

#[timed]
fn part2(input: &str) -> usize {
    let grid = Grid::new_from_input(input);
    let height = grid.grid.len();
    let width = grid.grid[0].len();

    let vertical_start_configurations =
        (0..height).flat_map(|start_y| [((start_y, 0), (0, 1)), ((start_y, width - 1), (0, -1))]);

    let horizontal_start_configurations =
        (0..width).flat_map(|start_x| [((0, start_x), (1, 0)), ((height - 1, start_x), (-1, 0))]);

    vertical_start_configurations
        .chain(horizontal_start_configurations)
        .par_bridge()
        .map(|(position, vector)| grid.shoot_from_number_of_energized(position, vector))
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
