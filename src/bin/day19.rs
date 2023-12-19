use std::{cmp::Ordering, collections::HashMap};

use aoc2023::{read_input, InputType};
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 19;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
    fn get_variable(&self, rating: &RatingVariable) -> usize {
        match rating {
            RatingVariable::X => self.x,
            RatingVariable::M => self.m,
            RatingVariable::A => self.a,
            RatingVariable::S => self.s,
        }
    }

    fn apply_rules(&self, rules: &[Rule], default: &WorkflowOrDone) -> WorkflowOrDone {
        for rule in rules {
            let rating_value = self.get_variable(&rule.rating);
            let matched = match rule.ordering {
                Ordering::Less => rating_value < rule.rating_value,
                Ordering::Greater => rating_value > rule.rating_value,
                Ordering::Equal => unreachable!(),
            };
            if matched {
                return rule.destination.clone();
            }
        }
        return default.clone();
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default_gateway: WorkflowOrDone,
}

#[derive(Clone)]
enum RatingVariable {
    X,
    M,
    A,
    S,
}

impl From<char> for RatingVariable {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
enum WorkflowOrDone {
    WorkflowName(String),
    Accept,
    Reject,
}

impl From<&str> for WorkflowOrDone {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::WorkflowName(s.to_string()),
        }
    }
}

#[derive(Clone)]
struct Rule {
    rating: RatingVariable,
    rating_value: usize,
    ordering: Ordering,
    destination: WorkflowOrDone,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (rule, destination) = value.split(":").collect_tuple().unwrap();
        let destination = WorkflowOrDone::from(destination);
        let rating = RatingVariable::from(rule.chars().nth(0).unwrap());
        let ordering = match rule.chars().nth(1).unwrap() {
            '<' => Ordering::Less,
            '>' => Ordering::Greater,
            _ => unreachable!(),
        };
        let rating_value = rule[2..].parse::<usize>().unwrap();

        Rule { rating, rating_value, ordering, destination }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<MachinePart>) {
    let (workflows_part, machine_part_part) =
        input.trim().split("\r\n\r\n").collect_tuple().unwrap();

    let machine_parts = machine_part_part
        .trim()
        .lines()
        .map(|line| {
            let (x, m, a, s) = line
                .trim()
                .trim_start_matches("{")
                .trim_end_matches("}")
                .split(",")
                .map(|rating_str| rating_str.split("=").last().unwrap().parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            MachinePart { x, m, a, s }
        })
        .collect_vec();

    let workflows = workflows_part
        .trim()
        .lines()
        .map(|line| {
            let (name, rest) = line.trim().split("{").collect_tuple().unwrap();
            let mut rest_vec = rest.trim_end_matches("}").split(",").collect_vec();
            let default_gateway = WorkflowOrDone::from(rest_vec.remove(rest_vec.len() - 1));
            let rules = rest_vec.iter().copied().map(Rule::from).collect_vec();
            (name.to_string(), Workflow { rules, default_gateway })
        })
        .fold(HashMap::new(), |mut map, (name, workflow)| {
            map.insert(name, workflow);
            map
        });

    (workflows, machine_parts)
}

fn process_machine_part(workflows: &HashMap<String, Workflow>, machine_part: &MachinePart) -> bool {
    let mut current_step = WorkflowOrDone::WorkflowName("in".to_string());

    while let WorkflowOrDone::WorkflowName(ref workflow_name) = current_step {
        let current_workflow = workflows.get(workflow_name).unwrap();
        current_step =
            machine_part.apply_rules(&current_workflow.rules, &current_workflow.default_gateway);
    }

    match current_step {
        WorkflowOrDone::Accept => true,
        WorkflowOrDone::Reject => false,
        WorkflowOrDone::WorkflowName(_) => unreachable!(),
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let (workflows, machine_parts) = parse_input(input);

    machine_parts
        .iter()
        .filter(|machine_part| process_machine_part(&workflows, machine_part))
        .map(|machine_part| machine_part.x + machine_part.m + machine_part.a + machine_part.s)
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
    fn part1_test() {
        let expected = 19114;
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
