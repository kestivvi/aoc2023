use std::collections::HashMap;

use aoc2023::{read_input, InputType};
use timed::timed;

const DAY: u8 = 20;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

struct Machine {
    button: usize,
    broadcaster: usize,
    modules: HashMap<String, Box<dyn Module>>,
}

impl Machine {
    fn new_from_input(input: &str) -> Self {
        // Parse input to Vec<(module_name, module_type, Vec<String>)>
        //                                               ^-- outputs

        // Based on parsed input create all modules, fill only module_name and type,
        // outputs and inputs will be processed later
        // let modules: HashMap<String, Box<dyn Module>>

        // Iterate mut over all parsed input again, but now add inputs and outputs to created modules
        todo!()
    }
}

#[derive(Clone, Copy)]
enum Pulse {
    High,
    Low,
}

trait Module {
    fn add_output(&mut self, input_name: &str);

    fn add_input(&mut self, input_name: &str);

    fn process_signal(&mut self, pulse: Pulse) -> Vec<(String, Pulse)>;
}

#[timed]
fn part1(input: &str) -> usize {
    todo!()
}

#[timed]
fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test1() {
        let expected = 32000000;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test2() {
        let expected = 11687500;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 0;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
