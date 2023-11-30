use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 0;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[timed]
fn part1(input: &str) -> String {
    todo!()
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
    fn test_part1() {
        let expected = "";
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = "";
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
