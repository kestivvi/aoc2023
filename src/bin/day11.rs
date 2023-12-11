use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 11;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input, 1_000_000));
}

struct Universe(Vec<Vec<Tile>>);

impl Universe {
    fn new(input: &str) -> Self {
        let universe_vec = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Tile {
                        object: c.try_into().unwrap(),
                        vertical_cost: 1,
                        horizontal_cost: 1,
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self(universe_vec)
    }

    fn expand(&mut self, expand_rate: u64) {
        self.0
            .iter_mut()
            .filter(|line| line.iter().all(|tile| tile.object == Object::Space))
            .for_each(|line| {
                line.iter_mut()
                    .for_each(|tile| tile.vertical_cost *= expand_rate)
            });

        let rows = self.0.len();
        let columns = self.0[0].len();

        let mut transposed_universe = (0..columns)
            .map(|column| {
                (0..rows)
                    .map(|row| self.0[row][column].clone())
                    .collect_vec()
            })
            .collect_vec();

        transposed_universe
            .iter_mut()
            .filter(|line| line.iter().all(|tile| tile.object == Object::Space))
            .for_each(|line| {
                line.iter_mut()
                    .for_each(|tile| tile.horizontal_cost *= expand_rate)
            });

        let final_universe = (0..columns)
            .map(|column| {
                (0..rows)
                    .map(|row| transposed_universe[row][column].clone())
                    .collect_vec()
            })
            .collect_vec();

        self.0 = final_universe;
    }

    fn path_from_to(&self, start: (usize, usize), end: (usize, usize)) -> u64 {
        let mut counter = 0;
        let mut current = start;

        while current != end {
            match current.0 as i64 - end.0 as i64 {
                x if x > 0 => {
                    counter += self.0[current.0][current.1].vertical_cost;
                    current.0 -= 1;
                }
                x if x < 0 => {
                    counter += self.0[current.0][current.1].vertical_cost;
                    current.0 += 1;
                }
                _ => (),
            }

            match current.1 as i64 - end.1 as i64 {
                x if x > 0 => {
                    counter += self.0[current.1][current.1].horizontal_cost;
                    current.1 -= 1;
                }
                x if x < 0 => {
                    counter += self.0[current.1][current.1].horizontal_cost;
                    current.1 += 1;
                }
                _ => (),
            }
        }

        counter
    }

    fn find_galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();

        for (y, line) in self.0.iter().enumerate() {
            for (x, &ref tile) in line.iter().enumerate() {
                if tile.object == Object::Galaxy {
                    galaxies.push((y, x));
                }
            }
        }

        galaxies
    }
}

#[derive(Clone)]
struct Tile {
    object: Object,
    vertical_cost: u64,
    horizontal_cost: u64,
}

#[derive(PartialEq, Clone)]
enum Object {
    Space,
    Galaxy,
}

impl TryFrom<char> for Object {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Galaxy),
            '.' => Ok(Self::Space),
            _ => unreachable!(),
        }
    }
}

#[timed]
fn part1(input: &str) -> u64 {
    let mut universe = Universe::new(input);
    universe.expand(2);

    universe
        .find_galaxies()
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let g1 = galaxies[0];
            let g2 = galaxies[1];

            universe.path_from_to(*g1, *g2)
        })
        .sum::<u64>()
}

#[timed]
fn part2(input: &str, expand_rate: usize) -> u64 {
    let mut universe = Universe::new(input);
    universe.expand(expand_rate.try_into().unwrap());

    universe
        .find_galaxies()
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let g1 = galaxies[0];
            let g2 = galaxies[1];

            universe.path_from_to(*g1, *g2)
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 374;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test1() {
        let expected = 1030;
        let result = part2(&get_test_input(), 10);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let expected = 8410;
        let result = part2(&get_test_input(), 100);
        assert_eq!(result, expected);
    }
}
