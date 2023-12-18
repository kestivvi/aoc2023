use std::collections::{HashMap, HashSet};

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 17;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec())
        .collect_vec()
}

fn find_cost(input: &str, min_distance: usize, max_distance: usize) -> usize {
    let grid = parse_input(input);
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let if_in_bounds = |y: isize, x: isize| -> bool {
        y >= 0 && y < grid_height as isize && x >= 0 && x < grid_width as isize
    };

    let mut queue: Vec<(usize, usize, usize, isize)> = Vec::new();
    queue.push((0, 0, 0, -1));

    let mut seen: HashSet<(usize, usize, isize)> = HashSet::new();
    let mut costs: HashMap<(usize, usize, isize), usize> = HashMap::new();

    while !queue.is_empty() {
        queue.sort_by_key(|(cost, _, _, _)| *cost);
        let (cost, x, y, dd) = queue.remove(0);

        if x == grid_width - 1 && y == grid_height - 1 {
            return cost;
        }

        if seen.contains(&(x, y, dd)) {
            continue;
        }

        seen.insert((x, y, dd));

        for direction in 0..4 {
            let mut cost_increase = 0;

            if direction == dd || (direction + 2) % 4 == dd {
                continue;
            }

            for distance in 1..=max_distance {
                let xx = x as isize + DIRECTIONS[direction as usize].0 * distance as isize;
                let yy = y as isize + DIRECTIONS[direction as usize].1 * distance as isize;

                if !if_in_bounds(yy, xx) {
                    continue;
                }

                let yy = yy as usize;
                let xx = xx as usize;

                cost_increase += grid[yy][xx];

                if distance < min_distance {
                    continue;
                }

                let new_cost = cost + cost_increase;

                if costs.get(&(xx, yy, direction)).is_some_and(|v| v <= &new_cost) {
                    continue;
                }

                costs.insert((xx, yy, direction), new_cost);
                queue.push((new_cost, xx, yy, direction));
            }
        }
    }

    unreachable!()
}

#[timed]
fn part1(input: &str) -> usize {
    find_cost(input, 1, 3)
}

#[timed]
fn part2(input: &str) -> usize {
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
        let expected = 102;
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
