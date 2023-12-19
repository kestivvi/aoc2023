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

#[derive(Clone)]
struct MachinePartRange {
    x_start: usize,
    x_end: usize,

    m_start: usize,
    m_end: usize,

    a_start: usize,
    a_end: usize,

    s_start: usize,
    s_end: usize,
}

impl MachinePartRange {
    fn distinct_possibilities(&self) -> u128 {
        let x = (self.x_end - self.x_start + 1) as u128;
        let m = (self.m_end - self.m_start + 1) as u128;
        let a = (self.a_end - self.a_start + 1) as u128;
        let s = (self.s_end - self.s_start + 1) as u128;

        x * m * a * s
    }

    fn get_variable(&self, rating: &RatingVariable) -> (usize, usize) {
        match rating {
            RatingVariable::X => (self.x_start, self.x_end),
            RatingVariable::M => (self.m_start, self.m_end),
            RatingVariable::A => (self.a_start, self.a_end),
            RatingVariable::S => (self.s_start, self.s_end),
        }
    }

    fn change_variable_start(&mut self, rating: &RatingVariable, value: usize) {
        match rating {
            RatingVariable::X => self.x_start = value,
            RatingVariable::M => self.m_start = value,
            RatingVariable::A => self.a_start = value,
            RatingVariable::S => self.s_start = value,
        };
    }

    fn change_variable_end(&mut self, rating: &RatingVariable, value: usize) {
        match rating {
            RatingVariable::X => self.x_end = value,
            RatingVariable::M => self.m_end = value,
            RatingVariable::A => self.a_end = value,
            RatingVariable::S => self.s_end = value,
        };
    }

    fn apply_rule(&self, rule: &Rule) -> Vec<(bool, MachinePartRange)> {
        let rating_value = self.get_variable(&rule.rating);
        match rule.ordering {
            Ordering::Less => {
                if rating_value.0 >= rule.rating_value {
                    vec![(false, self.clone())]
                } else if rating_value.1 < rule.rating_value {
                    vec![(true, self.clone())]
                } else {
                    let mut lower_range = self.clone();
                    let mut upper_range = self.clone();

                    lower_range.change_variable_end(&rule.rating, rule.rating_value - 1);
                    upper_range.change_variable_start(&rule.rating, rule.rating_value);

                    vec![(true, lower_range), (false, upper_range)]
                }
            }
            Ordering::Greater => {
                if rating_value.1 <= rule.rating_value {
                    vec![(false, self.clone())]
                } else if rule.rating_value < rating_value.0 {
                    vec![(true, self.clone())]
                } else {
                    let mut lower_range = self.clone();
                    let mut upper_range = self.clone();

                    lower_range.change_variable_end(&rule.rating, rule.rating_value);
                    upper_range.change_variable_start(&rule.rating, rule.rating_value + 1);

                    vec![(false, lower_range), (true, upper_range)]
                }
            }
            Ordering::Equal => unreachable!(),
        }
    }

    fn apply_workflow(
        machine_part_range: &MachinePartRange,
        workflow: &Workflow,
    ) -> Vec<(WorkflowOrDone, MachinePartRange)> {
        let mut result = vec![];
        let mut current_machine_part_range = machine_part_range.clone();

        for rule in &workflow.rules {
            let ranges = current_machine_part_range.apply_rule(rule);
            let ranges_len = ranges.len();
            for (meet_requirement, range) in ranges {
                if meet_requirement {
                    result.push((rule.destination.clone(), range));
                    if ranges_len == 1 {
                        return result;
                    }
                } else {
                    current_machine_part_range = range;
                }
            }
        }

        result.push((workflow.default_gateway.clone(), current_machine_part_range));

        result
    }

    fn apply_workflows(
        machine_part_range: &MachinePartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<MachinePartRange> {
        let mut queue: Vec<(String, MachinePartRange)> =
            vec![("in".to_string(), machine_part_range.clone())];

        let mut done = vec![];

        while !queue.is_empty() {
            let (current_workflow_name, current_machine_part_range) = queue.pop().unwrap();
            let current_workflow = workflows.get(&current_workflow_name).unwrap();
            let ranges =
                MachinePartRange::apply_workflow(&current_machine_part_range, current_workflow);

            for (workflow_or_done, range) in ranges {
                match workflow_or_done {
                    WorkflowOrDone::WorkflowName(s) => queue.push((s, range)),
                    WorkflowOrDone::Accept => done.push(range),
                    WorkflowOrDone::Reject => (),
                }
            }
        }

        done
    }
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
fn part2(input: &str) -> u128 {
    let (workflows, _) = parse_input(input);

    let initial_machine_part_range = MachinePartRange {
        x_start: 1,
        x_end: 4000,
        m_start: 1,
        m_end: 4000,
        a_start: 1,
        a_end: 4000,
        s_start: 1,
        s_end: 4000,
    };

    let accepted_machine_part_ranges =
        MachinePartRange::apply_workflows(&initial_machine_part_range, &workflows);

    accepted_machine_part_ranges.iter().map(|range| range.distinct_possibilities()).sum()
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
        let expected = 167409079868000;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
