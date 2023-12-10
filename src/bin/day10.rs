use std::collections::HashSet;

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use num::CheckedSub;
use timed::timed;

const DAY: u8 = 10;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Debug)]
enum Tile {
    Nothing,
    Pipe(Pipe),
}

impl Tile {
    fn is_connection(&self, to: Direction) -> bool {
        match self {
            Tile::Nothing => false,
            Tile::Pipe(pipe) => pipe.exits.iter().any(|exit| *exit == to),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Nothing),
            _ => Ok(Self::Pipe(value.try_into().unwrap())),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    exits: [Direction; 2],
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe {
                exits: [Direction::North, Direction::South],
            }),
            '-' => Ok(Pipe {
                exits: [Direction::West, Direction::East],
            }),
            'L' => Ok(Pipe {
                exits: [Direction::North, Direction::East],
            }),
            'J' => Ok(Pipe {
                exits: [Direction::North, Direction::West],
            }),
            '7' => Ok(Pipe {
                exits: [Direction::West, Direction::South],
            }),
            'F' => Ok(Pipe {
                exits: [Direction::East, Direction::South],
            }),
            'S' => Ok(Pipe {
                exits: [Direction::Unknown, Direction::Unknown],
            }),
            '.' => Err(()),
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    Unknown,
}

impl Direction {
    fn vector(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::Unknown => unreachable!(),
        }
    }
}

fn parse(input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut chars_grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut s_y = 0;
    let mut s_x = 0;

    'outer: for y in 0..chars_grid.len() {
        for x in 0..chars_grid[y].len() {
            if let Tile::Nothing = chars_grid[y][x] {
                continue;
            }

            let s_char = chars_grid[y][x].is_connection(Direction::Unknown);

            if s_char {
                s_y = y;
                s_x = x;

                let north = y.checked_sub(1).is_some()
                    && chars_grid[y - 1][x].is_connection(Direction::South);
                let south = y + 1 < chars_grid.len()
                    && chars_grid[y + 1][x].is_connection(Direction::North);
                let west = x.checked_sub(1).is_some()
                    && chars_grid[y][x - 1].is_connection(Direction::East);
                let east = x + 1 < chars_grid[y].len()
                    && chars_grid[y][x + 1].is_connection(Direction::West);

                let mut directions = vec![];
                if north {
                    directions.push(Direction::North);
                }

                if south {
                    directions.push(Direction::South);
                }

                if west {
                    directions.push(Direction::West);
                }

                if east {
                    directions.push(Direction::East);
                }

                chars_grid[y][x] = Tile::Pipe(Pipe {
                    exits: directions.try_into().unwrap(),
                });

                break 'outer;
            }
        }
    }

    ((s_y, s_x), chars_grid)
}

fn loop_positions(start: (usize, usize), grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut positions = vec![start];

    while positions.len() == 1 || (*positions.last().unwrap() != start) {
        let current = positions.last().unwrap();

        if let Tile::Pipe(pipe) = &grid[current.0][current.1] {
            let next_position = pipe
                .exits
                .iter()
                .map(|exit| {
                    let vector = exit.vector();
                    let next_position = (
                        (current.0 as i64 + vector.0) as usize,
                        (current.1 as i64 + vector.1) as usize,
                    );
                    next_position
                })
                .find(|next_position| {
                    positions.len() <= 1 || next_position != positions.iter().nth_back(1).unwrap()
                })
                .unwrap();

            positions.push(next_position);
        }
    }

    positions
}

#[timed]
fn part1(input: &str) -> u64 {
    let (start, grid) = parse(input);

    let positions = loop_positions(start, &grid);

    (positions.len() / 2) as u64
}

fn area_dfs(grid: &Vec<Vec<Tile>>, y: usize, x: usize, visited: &mut HashSet<(usize, usize)>) {
    if visited.contains(&(y, x)) {
        return;
    }

    if let Tile::Pipe(_) = grid[y][x] {
        return;
    }

    visited.insert((y, x));
    y.checked_sub(1).and_then(|y| {
        area_dfs(grid, y, x, visited);
        Some(())
    });

    if y + 1 < grid.len() {
        area_dfs(grid, y + 1, x, visited);
    }

    x.checked_sub(1).and_then(|x| {
        area_dfs(grid, y, x, visited);
        Some(())
    });

    if x + 1 < grid[y].len() {
        area_dfs(grid, y, x + 1, visited);
    }
}

// #[timed]
// fn part2(input: &str) -> u64 {
//     let (start, grid) = parse(input);
//     let positions = loop_positions(start, &grid);

//     let grid = grid
//         .into_iter()
//         .enumerate()
//         .map(|(y, line)| {
//             line.into_iter()
//                 .enumerate()
//                 .map(|(x, tile)| {
//                     let current_position = (y, x);
//                     if positions.iter().any(|pos| current_position == *pos) {
//                         tile
//                     } else {
//                         Tile::Nothing
//                     }
//                 })
//                 .collect_vec()
//         })
//         .collect_vec();

//     let mut areas: Vec<HashSet<(usize, usize)>> = vec![];

//     for y in 0..grid.len() {
//         for x in 0..grid[y].len() {
//             if let Tile::Pipe(_) = grid[y][x] {
//                 continue;
//             }

//             let visited = areas.iter().any(|area| area.contains(&(y, x)));
//             if visited {
//                 continue;
//             }

//             let mut new_area = HashSet::new();
//             area_dfs(&grid, y, x, &mut new_area);

//             if new_area.len() != 0 {
//                 areas.push(new_area);
//             }
//         }
//     }

//     areas
//         .iter()
//         .map(|area| area.len())
//         .sorted()
//         .rev()
//         .skip(1)
//         .sum::<usize>() as u64
// }

#[timed]
fn part2(input: &str) -> u64 {
    let (start, grid) = parse(input);
    let positions = loop_positions(start, &grid);

    let grid = grid
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, tile)| {
                    let current_position = (y, x);
                    if positions.iter().any(|pos| current_position == *pos) {
                        tile
                    } else {
                        Tile::Nothing
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut area = 0;
    let mut counter = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Tile::Pipe(_) = grid[y][x] {
                if y.checked_sub(1).is_some() {
                    if let Tile::Pipe(_) = grid[y - 1][x] {
                        let step = positions
                    }
                }
                continue;
            }

            let visited = areas.iter().any(|area| area.contains(&(y, x)));
            if visited {
                continue;
            }

            let mut new_area = HashSet::new();
            area_dfs(&grid, y, x, &mut new_area);

            if new_area.len() != 0 {
                areas.push(new_area);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let expected = 4;
        let result = part1(&read_input(DAY, InputType::Other("test1".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test2() {
        let expected = 4;
        let result = part1(&read_input(DAY, InputType::Other("test2".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test3() {
        let expected = 8;
        let result = part1(&read_input(DAY, InputType::Other("test3".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test4() {
        let expected = 8;
        let result = part1(&read_input(DAY, InputType::Other("test4".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_wj() {
        let expected = 6860;
        let result = part1(&read_input(DAY, InputType::Other("wj".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    // #[test]
    // fn part2_test() {
    //     let expected = "";
    //     let result = part2(&get_test_input());
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn part2_test1() {
        let expected = 1;
        let result = part2(&read_input(DAY, InputType::Other("test1".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let expected = 1;
        let result = part2(&read_input(DAY, InputType::Other("test2".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test5() {
        let expected = 4;
        let result = part2(&read_input(DAY, InputType::Other("test5".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test6() {
        let expected = 4;
        let result = part2(&read_input(DAY, InputType::Other("test6".to_owned())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test7() {
        let expected = 8;
        let result = part2(&read_input(DAY, InputType::Other("test7".to_owned())).unwrap());
        assert_eq!(result, expected);
    }
}
