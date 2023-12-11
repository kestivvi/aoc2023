use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 11;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input, 1_000_000));
}

fn print_universe(universe: &Vec<Vec<char>>) {
    universe.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{c}"));
        println!();
    })
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn expand_rows(universe: &mut Vec<Vec<char>>, expand_rate: usize) {
    let rows_to_expand = universe
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect_vec();

    let mut counter = 0;
    rows_to_expand.iter().for_each(|&row_index| {
        let row_to_copy = universe[row_index + counter].clone();

        let expand_rate = if expand_rate > 1 {
            expand_rate - 1
        } else {
            expand_rate
        };

        for _ in 0..expand_rate {
            universe.insert(row_index + counter, row_to_copy.clone());
            counter += 1;
        }
    });
}

fn expand_universe(input: &str, expand_rate: usize) -> Vec<Vec<char>> {
    let mut universe = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // println!("Raw");
    // print_universe(&universe);

    expand_rows(&mut universe, expand_rate);

    // println!("Expanded rows");
    // print_universe(&universe);

    let mut universe = transpose2(universe);
    // println!("Transposed");
    // print_universe(&universe);

    expand_rows(&mut universe, expand_rate);
    // println!("Expanded rows");
    // print_universe(&universe);

    let final_universe = transpose2(universe);

    // println!("Transposed again");
    // print_universe(&final_universe);
    final_universe
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let galaxy_char = '#';

    for (y, line) in universe.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == galaxy_char {
                galaxies.push((y, x));
            }
        }
    }

    galaxies
}

#[timed]
fn part1(input: &str) -> u64 {
    let universe = expand_universe(input, 1);
    let galaxies = find_galaxies(&universe);

    galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let g1 = galaxies[0];
            let g2 = galaxies[1];

            (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as u64
        })
        .sum::<u64>()
}

#[timed]
fn part2(input: &str, expand_rate: usize) -> u64 {
    let universe = expand_universe(input, expand_rate);
    let galaxies = find_galaxies(&universe);

    galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let g1 = galaxies[0];
            let g2 = galaxies[1];

            (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as u64
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
