use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::vec;
use regex::Regex;
use timed::timed;

const DAY: u8 = 12;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (conditions, groups) = line.trim().split(' ').collect_tuple().unwrap();

            let conditions = conditions.chars().collect_vec();
            let groups = groups
                .split(',')
                .flat_map(|number| number.parse())
                .collect_vec();

            (conditions, groups)
        })
        .collect_vec()
}

fn is_valid(conditions: &Vec<char>, groups: &Vec<usize>) -> bool {
    let re = Regex::new("#+").unwrap();
    let x = conditions.iter().collect::<String>();
    let y = re.captures_iter(&x).collect_vec();

    if y.len() != groups.len() {
        return false;
    }

    let z = y
        .iter()
        .zip(groups.iter())
        .all(|(capture, group)| capture[0].len() == *group);

    // if z {
    //     dbg!((conditions, groups, y, z));
    // }

    z
}

fn dfs(conditions: Vec<char>, index: usize, acc: &mut Vec<Vec<char>>) {
    if index >= conditions.len() {
        acc.push(conditions);
        return;
    }

    if conditions[index] != '?' {
        dfs(conditions, index + 1, acc);
        return;
    }

    let mut copy = conditions;
    copy[index] = '#';
    dfs(copy.clone(), index + 1, acc);
    copy[index] = '.';
    dfs(copy, index + 1, acc);
}

fn combinations(conditions: &Vec<char>, groups: &Vec<usize>) -> usize {
    let number_of_unknowns = conditions.iter().filter(|&&c| c == '?').count();
    let mut all: Vec<Vec<char>> = Vec::with_capacity(number_of_unknowns);

    dfs(conditions.clone(), 0, &mut all);

    all.iter().filter(|guess| is_valid(guess, groups)).count()
}

#[timed]
fn part1(input: &str) -> usize {
    let records = parse(input);

    records
        .iter()
        .map(|(conditions, groups)| combinations(conditions, groups))
        .sum()
}

#[timed]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn is_valid_test1() {
        let expected = true;
        let result = is_valid(&vec!['.', '#', '#', '#', '.', '#'], &vec![3, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test() {
        let expected = 21;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = "";
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
