use aoc2023::{read_input, InputType};
use itertools::Itertools;
use std::io::{self, Read};
use std::str::FromStr;

const DAY: u8 = 5;

fn main() {
    let expected = 125742456;
    let result = part2(&read_input(DAY, InputType::Real).unwrap());
    assert_eq!(result, expected);
}

#[timed::timed]
fn part2(input: &str) -> i64 {
    let mut blocks = input.split("\r\n\r\n");

    let mut seeds = blocks
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .tuples()
        .map(|(a, b)| (a, a + b))
        .collect::<Vec<_>>();

    for block in blocks {
        let ranges = block
            .split("\r\n")
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let mut new = Vec::with_capacity(seeds.len());
        while let Some((s, e)) = seeds.pop() {
            let was_overlap = ranges.iter().any(|&(a, b, c)| {
                let os = s.max(b);
                let oe = e.min(b + c);
                if os < oe {
                    new.push((os - b + a, oe - b + a));
                    if os > s {
                        seeds.push((s, os));
                    }
                    if e > oe {
                        seeds.push((oe, e));
                    }
                    true
                } else {
                    false
                }
            });
            if !was_overlap {
                new.push((s, e));
            }
        }
        seeds = new;
    }

    seeds.iter().map(|&(s, _)| s).min().unwrap()
}

#[cfg(test)]
mod tests {
    use aoc2023::InputType;

    use super::*;

    #[test]
    fn part2_test() {
        let expected = 46;
        let result = part2(&read_input(DAY, InputType::Test).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_real() {
        let expected = 125742456;
        let result = part2(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj() {
        let expected = 20358599;
        let result = part2(&read_input(DAY, InputType::Other("WJ".to_string())).unwrap());
        assert_eq!(result, expected);
    }
}
