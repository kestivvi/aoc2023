use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 15;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

fn get_hash(sequence: &str) -> usize {
    sequence
        .trim()
        .chars()
        .fold(0, |acc, c| (((c as u8) as usize + acc) * 17) % 256)
}

#[timed]
fn part1(input: &str) -> usize {
    input.trim().split(',').map(get_hash).sum()
}

#[timed]
fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input.trim().split(',').for_each(|sequence| {
        let lens_name = sequence
            .split('=')
            .next()
            .unwrap()
            .split('-')
            .next()
            .unwrap();
        let box_index = get_hash(lens_name);
        let r#box = boxes.get_mut(box_index).unwrap();
        let position = r#box
            .iter()
            .position(|(lens_in_box_name, _)| *lens_in_box_name == lens_name);

        if sequence.contains('-') && position.is_some() {
            r#box.remove(position.unwrap());
        }

        if sequence.contains('=') {
            let (_, power) = sequence.split('=').collect_tuple().unwrap();
            let power = power.parse::<usize>().unwrap();
            let new_value = (lens_name, power);

            match position {
                Some(p) => r#box[p] = new_value,
                None => r#box.push(new_value),
            };
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_index, r#box)| {
            r#box
                .iter()
                .enumerate()
                .map(|(index_in_box, (_, power))| (box_index + 1) * (index_in_box + 1) * power)
                .sum::<usize>()
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
    fn part1_test() {
        let expected = 1320;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let expected = 507291;
        let result = part1(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 145;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_real() {
        let expected = 296921;
        let result = part2(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }
}
