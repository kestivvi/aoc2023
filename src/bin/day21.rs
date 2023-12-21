use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::vec;
use timed::timed;

const DAY: u8 = 21;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input, 64));
    println!("Part2: {}", part2(&real_input, 26501365));
}

fn neighbours(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if y > 0 {
        neighbours.push((y - 1, x));
    }

    if x > 0 {
        neighbours.push((y, x - 1));
    }

    if y + 1 < grid.len() {
        neighbours.push((y + 1, x));
    }

    if x + 1 < grid[y].len() {
        neighbours.push((y, x + 1));
    }

    neighbours.into_iter().filter(|(y, x)| grid[*y][*x] != '#').collect_vec()
}

fn neighbours2(grid: &Vec<Vec<char>>, y: isize, x: isize) -> Vec<(isize, isize)> {
    let neighbours = vec![(y - 1, x), (y, x - 1), (y + 1, x), (y, x + 1)];

    neighbours
        .into_iter()
        .filter(|(y, x)| {
            let y = (y.rem_euclid(grid.len() as isize)) as usize;
            let x = (x.rem_euclid(grid[0].len() as isize)) as usize;
            grid[y][x] != '#'
        })
        .collect_vec()
}

#[timed]
fn part1(input: &str, steps: usize) -> usize {
    let mut grid = input.trim().lines().map(|line| line.chars().collect_vec()).collect_vec();
    let start_position = {
        let p = grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (y, x, c)))
            .find(|(y, x, c)| **c == 'S')
            .unwrap();
        (p.0, p.1)
    };

    grid[start_position.0][start_position.1] = '.';

    let mut queue: HashSet<(usize, usize)> = HashSet::new();
    queue.insert(start_position);

    for _ in 0..steps {
        let mut next_queue: HashSet<(usize, usize)> = HashSet::new();
        for (y, x) in queue {
            let neighbours = neighbours(&grid, y, x);
            next_queue.extend(neighbours);
        }
        queue = next_queue;
    }

    queue.len()
}

#[timed]
fn part2(input: &str, steps: usize) -> usize {
    let mut grid = input.trim().lines().map(|line| line.chars().collect_vec()).collect_vec();
    let start_position = {
        let p = grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (y, x, c)))
            .find(|(y, x, c)| **c == 'S')
            .unwrap();
        (p.0 as isize, p.1 as isize)
    };

    grid[start_position.0 as usize][start_position.1 as usize] = '.';

    let mut queue: HashSet<(isize, isize)> = HashSet::new();
    queue.insert(start_position);

    let mut numbers: Vec<(usize, usize)> = vec![];

    for step in 0..steps {
        let mut next_queue: HashSet<(isize, isize)> = HashSet::new();
        for (y, x) in queue {
            let neighbours = neighbours2(&grid, y, x);
            next_queue.extend(neighbours);
        }
        queue = next_queue;

        if queue.iter().contains(&(
            start_position.0 + ((numbers.len() + 1) * grid.len()) as isize,
            start_position.1,
        )) {
            numbers.push(dbg!((step, queue.len())));
        }

        if numbers.len() > 20 {
            dbg!(numbers);
            break;
        }
    }

    queue.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 16;
        let result = part1(&get_test_input(), 6);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 16733044;
        let result = part2(&get_test_input(), 5000);
        assert_eq!(result, expected);
    }
}
