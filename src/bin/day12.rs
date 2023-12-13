use std::{
    collections::HashMap,
    f32::consts::E,
    sync::{Arc, Mutex},
};

use aoc2023::{read_input, InputType};
use cached::proc_macro::cached;
use cached::*;
use counter::Counter;
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;
use regex::Regex;
use timed::timed;

const DAY: u8 = 12;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    // println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (conditions, groups) = line.trim().split(' ').collect_tuple().unwrap();

            // let conditions = vec!['.']
            //     .into_iter()
            //     .chain(conditions.chars())
            //     .chain(vec!['.'].into_iter())
            //     .collect_vec();
            let conditions = vec![].into_iter().chain(conditions.chars()).collect_vec();
            let groups = groups
                .split(',')
                .flat_map(|number| number.parse())
                .collect_vec();

            (conditions, groups)
        })
        .collect_vec()
}

fn is_valid(conditions: &str, groups: &Vec<usize>) -> bool {
    let re = Regex::new("#+").unwrap();
    let y = re.captures_iter(conditions).collect_vec();

    if y.len() != groups.len() {
        return false;
    }

    let z = y
        .iter()
        .zip(groups.iter())
        .all(|(capture, group)| capture[0].len() == *group);

    // if z {
    // dbg!((conditions, y, z));
    // }

    z
}

fn is_valid_so_far(conditions: &str, groups: &[usize]) -> bool {
    let new_conditions = conditions.replace(".", "..");
    let captures = Regex::new(r"\.(#+)\.")
        .unwrap()
        .captures_iter(&new_conditions)
        .map(|capture| capture[1].to_owned())
        .collect_vec();

    let mut frequency = captures.iter().map(|cap| cap.len()).collect::<Counter<_>>();

    let copy = frequency.clone();
    let keys = copy.keys();
    for k in keys {
        if !groups.contains(&(*k as usize)) {
            return false;
        }

        let value = frequency.entry(*k).or_default();

        if (*value as i32) < 0 {
            return false;
        }
    }

    let result = captures.iter().zip(groups.iter()).all(|(capture, group)| {
        // if conditions == "..#.#..###.." {
        //     dbg!((&conditions, &groups, &capture, &group));
        // }
        capture.len() == *group
    });

    // if conditions.len() > 38 {
    //     dbg!((conditions, result));
    // }
    result
}

fn dfs(conditions: Vec<char>, index: usize, acc: &mut Vec<Vec<char>>, groups: &[usize]) {
    if index >= conditions.len() {
        acc.push(conditions);
        return;
    }

    let mark_index = conditions.iter().position(|&c| c == '?');
    if let Some(mark_index) = mark_index {
        if !is_valid_so_far(
            &conditions[0..mark_index].iter().collect::<String>(),
            groups,
        ) {
            return;
        }
    }
    // dbg!((&conditions.iter().collect::<String>(), index));

    if conditions[index] != '?' {
        dfs(conditions, index + 1, acc, groups);
        return;
    }

    let mut copy = conditions;
    copy[index] = '#';
    dfs(copy.clone(), index + 1, acc, groups);

    copy[index] = '.';
    dfs(copy, index + 1, acc, groups);
}

fn combinations(conditions: &Vec<char>, groups: &Vec<usize>) -> usize {
    // let number_of_unknowns = conditions.iter().filter(|&&c| c == '?').count();
    let mut all: Vec<Vec<char>> = Vec::new();

    dfs(conditions.clone(), 0, &mut all, groups);

    all.iter()
        .filter(|guess| {
            let copied = guess.into_iter().copied().collect::<String>();
            is_valid(&copied, groups)
        })
        .count()
}

#[cached(
    key = "String",
    convert = r#"{ format!("{:?}{}{:?}", conditions, index, groups) }"#
)]
fn combinations2(conditions: &[char], index: usize, groups: &[usize]) -> usize {
    if groups.len() == 0 {
        if !conditions.iter().any(|&c| c == '#') {
            return 1;
        } else {
            return 0;
        }
    }

    if conditions.len() == 0 {
        return 0;
    }

    if index == conditions.len() {
        if index == groups[0] {
            let all_hashes = conditions[0..index].iter().all(|&c| c == '#');
            if all_hashes {
                return combinations2(&conditions[index..], 0, &groups[1..]);
            }
        }
        return 0;
    }

    if index == 0 && conditions[index] == '.' {
        return combinations2(&conditions[1..], 0, groups);
    }

    match conditions[index] {
        '.' => {
            if index == groups[0] && conditions[0..index].iter().all(|&c| c == '#') {
                return combinations2(&conditions[(index + 1)..], 0, &groups[1..]);
            } else {
                return 0;
            }
        }
        '#' => {
            if (index + 1) > groups[0] {
                return 0;
            } else {
                return combinations2(&conditions, index + 1, groups);
            }
        }
        '?' => {
            let mut copy1 = conditions.to_vec();
            copy1[index] = '.';
            let mut copy2 = copy1.clone();
            copy2[index] = '#';
            return combinations2(&copy1, index, groups) + combinations2(&copy2, index, groups);
        }
        _ => unreachable!(),
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let records = parse(input);

    let counter = Arc::new(Mutex::new(0));
    records
        .par_iter()
        .enumerate()
        .map(|(_, (conditions, groups))| {
            {
                let x = counter.clone();
                let mut y = x.lock().unwrap();
                *y += 1;
                println!("{y}");
            }
            combinations2(conditions, 0, groups)
        })
        .sum()
}

#[timed]
fn part2(input: &str) -> usize {
    let records = parse(input)
        .into_iter()
        .map(|(conditions, groups)| {
            let mut new_conditions = vec![];
            for i in 0..5 {
                if i != 0 {
                    new_conditions.push('?');
                }
                new_conditions.extend(conditions.iter());
            }

            let mut new_groups = vec![];
            for _ in 0..5 {
                new_groups.extend(groups.iter());
            }

            (new_conditions, new_groups)
        })
        .collect_vec();

    let counter = Arc::new(Mutex::new(0));
    records
        .iter()
        .enumerate()
        .map(|(index, (conditions, groups))| {
            // {
            //     let x = counter.clone();
            //     let mut y = x.lock().unwrap();
            //     *y += 1;
            //     println!("{y}");
            // }

            // dbg!(combinations(conditions, groups));
            // dbg!((conditions.iter().collect::<String>(), groups));
            let res = combinations2(conditions, 0, groups);
            // println!("{res}");
            res
        })
        .sum()
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
        let result = is_valid_so_far(".###.#.", &vec![3, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn is_valid_test2() {
        let expected = true;
        let result = is_valid_so_far(".###.#..##?", &vec![3, 1, 2]);
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
        let expected = 525152;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test1() {
        let expected = 1;
        let result = part2(&"???.### 1,1,3");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let expected = 1;
        let result = part1(&"? 1");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test3() {
        let expected = 2;
        let result = part1(&"?? 1");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test4() {
        let expected = 1;
        let result = part1(&"?? 2");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test5() {
        let expected = 3;
        let result = part1(&"??? 1");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test6() {
        let expected = 4;
        let result = part1(&"???? 1");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test7() {
        let expected = 3;
        let result = part1(&"???? 2");
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test8() {
        let expected = 3;
        let result = part1(&"???? 1,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1(".??..??...?##... 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("?#?#?#?#?#?#?#?.. 1,3,1,6"), 1);
        assert_eq!(part1("?#?#?#?#?#?#?#?... 1,3,1,6"), 1);
        assert_eq!(part1("????.#...#... 4,1,1"), 1);
        assert_eq!(part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(part1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn part2_wj() {
        let expected = 6555315065024;
        let result = part2(&read_input(DAY, InputType::Other("wj".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj1() {
        let expected = 1;
        let result = part2(&".?##.?.?#? 3,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let expected = 7260;
        let result = part1(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }
}
