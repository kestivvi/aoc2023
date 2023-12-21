use aoc2023::{read_input, InputType};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use timed::timed;

const DAY: u8 = 20;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

struct Machine {
    broadcaster: Box<dyn Module>,
    modules: HashMap<String, Box<dyn Module>>,
    low_pulses: usize,
    high_pulses: usize,
}

impl Machine {
    fn new_from_input(input: &str) -> Self {
        // Parse input to Vec<(module_name, module_type, Vec<String>)>
        //                                               ^-- outputs
        let parsed_input = input
            .trim()
            .lines()
            .map(|line| {
                let (module_part, outputs_part) = line.split(" -> ").collect_tuple().unwrap();
                let outputs = outputs_part
                    .trim()
                    .split(",")
                    .map(|output| output.trim().to_string())
                    .collect_vec();

                let module_type = if module_part.contains("%") {
                    ModuleType::FlipFlop
                } else if module_part.contains("&") {
                    ModuleType::Conjunction
                } else {
                    ModuleType::Broadcaster
                };

                let module_name = module_part.replace("%", "").replace("&", "").trim().to_string();

                (module_name, module_type, outputs)
            })
            .collect_vec();

        // Based on parsed input create all modules, fill only module_name and type,
        // outputs and inputs will be processed later
        // let modules: HashMap<String, Box<dyn Module>>

        let mut modules = parsed_input
            .iter()
            .map(|(module_name, module_type, _outputs)| {
                let module: Box<dyn Module> = match module_type {
                    ModuleType::FlipFlop => Box::new(FlipFlop::new(module_name.clone())),
                    ModuleType::Conjunction => Box::new(Conjunction::new(module_name.clone())),
                    ModuleType::Broadcaster => Box::new(Broadcaster::new(module_name.clone())),
                };

                (module, module_name)
            })
            .fold(HashMap::new(), |mut map, (module, name)| {
                map.insert(name.clone(), module);
                map
            });

        // dbg!(&modules);

        // Iterate mut over all parsed input again, but now add inputs and outputs to created modules
        for (module_name, _module_type, outputs) in parsed_input.iter() {
            let mut to_add = Vec::new();
            {
                let module = modules.get_mut(module_name).unwrap();
                for output in outputs.iter() {
                    module.add_output(output);
                    to_add.push((output.clone(), module_name.clone()));
                }
            }
            for (output, module_name) in to_add {
                let output_module = modules.get_mut(&output);
                match output_module {
                    Some(output_module) => output_module.add_input(&module_name),
                    None => (),
                };
            }
        }

        let broadcaster = modules.remove(&"broadcaster".to_string()).unwrap();

        Machine { broadcaster, modules, low_pulses: 0, high_pulses: 0 }
    }

    fn push_button(&mut self) -> bool {
        let mut queue = VecDeque::new();

        self.low_pulses += 1;

        let initial_pulses = self.broadcaster.process_signal(Pulse::Low, String::new());
        initial_pulses.iter().for_each(|(_, _, pulse)| match pulse {
            Pulse::High => self.high_pulses += 1,
            Pulse::Low => self.low_pulses += 1,
        });
        queue.extend(initial_pulses);

        while !queue.is_empty() {
            let (from, to, pulse) = queue.pop_front().unwrap();

            let module = match self.modules.get_mut(&to) {
                Some(module) => module,
                None => continue,
            };

            let sent_pulses = module.process_signal(pulse, from);

            sent_pulses.iter().for_each(|(_, _, pulse)| match pulse {
                Pulse::High => self.high_pulses += 1,
                Pulse::Low => self.low_pulses += 1,
            });

            if sent_pulses.iter().any(|(_, to, pulse)| *pulse == Pulse::Low && to == "rx") {
                println!("FOUND!");
                return true;
            }

            queue.extend(sent_pulses);
        }

        return false;
    }

    // fn get_state_of_zp_inputs(&self) -> Vec<bool> {
    //     self.modules.iter().map(|module| {
    //         module.1.get_outputs().iter().any(|(name, )|)
    //     })
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

trait Module {
    fn get_name(&self) -> String;

    // fn get_outputs(&self) -> Vec<(String, Option<bool>)>;

    fn add_output(&mut self, input_name: &str);

    fn add_input(&mut self, input_name: &str);

    fn process_signal(&mut self, pulse: Pulse, from: String) -> Vec<(String, String, Pulse)>;
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(name: String) -> Self {
        Self { name, outputs: vec![] }
    }
}

impl Module for Broadcaster {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn add_output(&mut self, input_name: &str) {
        self.outputs.push(input_name.to_string());
    }

    fn add_input(&mut self, _input_name: &str) {}

    fn process_signal(&mut self, pulse: Pulse, _from: String) -> Vec<(String, String, Pulse)> {
        self.outputs.iter().map(|output| (self.name.clone(), output.clone(), pulse)).collect_vec()
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(name: String) -> Self {
        Self { name, state: false, outputs: vec![] }
    }
}

impl Module for FlipFlop {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn add_output(&mut self, input_name: &str) {
        self.outputs.push(input_name.to_string());
    }

    fn add_input(&mut self, _input_name: &str) {}

    fn process_signal(&mut self, pulse: Pulse, _from: String) -> Vec<(String, String, Pulse)> {
        if let Pulse::High = pulse {
            return vec![];
        }

        let pulse_to_send = if self.state == false { Pulse::High } else { Pulse::Low };
        self.state = !self.state;

        self.outputs
            .iter()
            .map(|output| (self.name.clone(), output.clone(), pulse_to_send))
            .collect_vec()
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    inputs_last_pulse: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: String) -> Self {
        Self { name, inputs_last_pulse: HashMap::new(), outputs: vec![] }
    }
}

impl Module for Conjunction {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn add_output(&mut self, input_name: &str) {
        self.outputs.push(input_name.to_string());
    }

    fn add_input(&mut self, input_name: &str) {
        self.inputs_last_pulse.insert(input_name.to_string(), Pulse::Low);
    }

    fn process_signal(&mut self, pulse: Pulse, from: String) -> Vec<(String, String, Pulse)> {
        self.inputs_last_pulse.insert(from, pulse);

        if self.inputs_last_pulse.iter().all(|(_input, pulse)| *pulse == Pulse::High) {
            return self
                .outputs
                .iter()
                .map(|output| (self.name.clone(), output.clone(), Pulse::Low))
                .collect_vec();
        } else {
            return self
                .outputs
                .iter()
                .map(|output| (self.name.clone(), output.clone(), Pulse::High))
                .collect_vec();
        }
    }
}

#[timed]
fn part1(input: &str) -> usize {
    let mut machine = Machine::new_from_input(input);

    for _ in 0..1000 {
        machine.push_button();
    }

    machine.high_pulses * machine.low_pulses
}

#[timed]
fn part2(input: &str) -> usize {
    let mut machine = Machine::new_from_input(input);

    let mut i = 0;
    loop {
        i += 1;
        if i % 1000000 == 0 {
            println!("{i}");
        }
        if machine.push_button() {
            break;
        }
    }

    println!("Requires {i} pushes!");

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let expected = 32000000;
        let result = part1(&read_input(DAY, InputType::Other("test1".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test2() {
        let expected = 11687500;
        let result = part1(&read_input(DAY, InputType::Other("test2".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    // #[test]
    // fn part2_test() {
    //     let expected = 0;
    //     let result = part2(&get_test_input());
    //     assert_eq!(result, expected);
    // }
}
