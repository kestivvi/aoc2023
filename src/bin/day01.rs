use aoc2023::{read_input, InputType};
use itertools::Itertools;
use map_macro::hash_map;
use timed::timed;

const DAY: u8 = 1;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[timed]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let digits = line
                .chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            number.parse::<u32>()
        })
        .sum()
}

// fn replaceStrToDigits(input: &str) -> String {
//     let mut hello = hash_map! {
//         "one" => input.find("one"),
//         "two" => input.find("two"),
//         "three" => input.find("three"),
//         "four" => input.find("four"),
//         "five" => input.find("five"),
//         "six" => input.find("six"),
//         "seven" => input.find("seven"),
//         "eight" => input.find("eight"),
//         "nine" => input.find("nine"),
//     }
//     .iter()
//     .map(|(k, v)| v.map(|x| x as f32).or_else(|| Some(f32::INFINITY)).unwrap())
//     .sorted_by(|a, b| a.partial_cmp(b).unwrap());

//     let new_string = input.to_owned();

//     while hello.any(|v| v != f32::INFINITY) {
//         new_string.replace(from, to)
//     }
//     "".to_owned()
// }

fn convert1(line: &str, current_index: usize, str_num: &str, real_num: u32) -> Option<u32> {
    if ['1', '2', '3', '4', '5', '6', '7', '8', '9']
        .contains(&line.chars().nth(current_index).unwrap())
    {
        return line.chars().nth(current_index).unwrap().to_digit(10);
    }

    if let Some(index) = line.split_at(current_index).1.find(str_num) {
        // dbg!(index);
        if index == 0 {
            return Some(real_num);
        }
    }
    None
}

fn convert2(line: &str, current_index: usize) -> Option<u32> {
    // dbg!((line, current_index, line.chars().nth(current_index)));
    // dbg!(
    [
        convert1(line, current_index, "one", 1),
        convert1(line, current_index, "two", 2),
        convert1(line, current_index, "three", 3),
        convert1(line, current_index, "four", 4),
        convert1(line, current_index, "five", 5),
        convert1(line, current_index, "six", 6),
        convert1(line, current_index, "seven", 7),
        convert1(line, current_index, "eight", 8),
        convert1(line, current_index, "nine", 9),
    ]
    // )
    .iter()
    .flatten()
    .copied()
    .next()
}

fn my_replace(line: &str, current_index: usize, str_num: &str, real_num: &str) -> String {
    if let Some(index) = line.find(str_num) {
        if index == current_index {
            return line.replace(str_num, real_num);
        }
    }
    line.to_owned()
}

#[timed]
fn part2_v1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // println!("{}", line);
            // line.replace("one", "1")
            //     .replace("two", "2")
            //     .replace("three", "3")
            //     .replace("four", "4")
            //     .replace("five", "5")
            //     .replace("six", "6")
            //     .replace("seven", "7")
            //     .replace("eight", "8")
            //     .replace("nine", "9")
            let mut new_line = line.to_owned();
            for i in 0..new_line.len() {
                new_line = my_replace(&new_line, i, "one", "1");
                new_line = my_replace(&new_line, i, "two", "2");
                new_line = my_replace(&new_line, i, "three", "3");
                new_line = my_replace(&new_line, i, "four", "4");
                new_line = my_replace(&new_line, i, "five", "5");
                new_line = my_replace(&new_line, i, "six", "6");
                new_line = my_replace(&new_line, i, "seven", "7");
                new_line = my_replace(&new_line, i, "eight", "8");
                new_line = my_replace(&new_line, i, "nine", "9");
            }
            new_line
        })
        .flat_map(|line| {
            // println!("{}", line);
            let digits = line
                .chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            // println!("{}\n", number);
            number.parse::<u32>()
        })
        .sum()
}

#[timed]
fn part2_v2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // println!("{}", line);
            let mut digits = vec![];
            for i in 0..line.len() {
                digits.push(convert2(line, i));
            }

            digits.iter().flatten().copied().collect::<Vec<u32>>()
        })
        .flat_map(|digits| {
            // println!("{:?}", digits);
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            // println!("{}\n", number);
            // println!("{:?}", number);
            number.parse::<u32>()
        })
        .sum()
}

fn convert_to_digits(line: &str, current_index: usize) -> Option<u32> {
    let foo = |from: &str, to: u32| -> Option<u32> {
        let current_slice = line.split_at(current_index).1;
        if let Some(found) = current_slice.find(&format!("{}", to)) {
            if found == 0 {
                return Some(to);
            }
        }
        if let Some(index) = current_slice.find(from) {
            if index == 0 {
                return Some(to);
            }
        }
        None
    };
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .map(|(i, v)| foo(v, (i + 1) as u32))
    .flatten()
    .next()
}

#[timed]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let digits = (0..line.len())
                .map(|i| convert_to_digits(line, i))
                .flatten()
                .collect_vec();
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            number.parse::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input1() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    fn get_test_input2() -> String {
        read_input(DAY, InputType::Other("test2".to_owned())).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 142;
        let result = part1(&get_test_input1());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 281;
        let result = part2(&get_test_input2());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_dp() {
        let expected = 54728;
        let input = read_input(DAY, InputType::Other("DP".to_owned())).unwrap();
        let result = part2(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_mn() {
        let expected = 53389;
        let input = read_input(DAY, InputType::Other("MN".to_owned())).unwrap();
        let result = part2(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj() {
        let expected = 54265;
        let input = read_input(DAY, InputType::Other("WJ".to_owned())).unwrap();
        let result = part2(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fancy_input() {
        let input = "sevenine";
        assert_eq!(part2(&input), 79);
    }

    #[test]
    fn test_fancy2_input() {
        let input = "dkmmzhbvq3three6threeq";
        assert_eq!(part2(&input), 33);
    }

    #[test]
    fn test_fancy3_input() {
        let input = "sbzvkxclj33zgfrqrv";
        assert_eq!(part2(&input), 33);
    }
}
