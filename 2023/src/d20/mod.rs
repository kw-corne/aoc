use std::collections::{HashMap, VecDeque};
use std::path::Path;

use crate::util::get_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    CJ,
    FF,
    BC,
}

#[derive(Debug)]
struct Module {
    name: String,
    mtype: ModuleType,
    on: bool,
    inputs: Vec<(String, Pulse)>,
    outputs: Vec<String>,
}

impl Module {
    fn new(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().collect();
        let raw = words[0];
        let outputs: Vec<String> = words[2..]
            .iter()
            .map(|s| s.trim_end_matches(',').to_string())
            .collect();
        let (name, mtype) = {
            match raw.chars().nth(0).unwrap() {
                'b' => ("broadcaster".to_string(), ModuleType::BC),
                '%' => (raw[1..].to_string(), ModuleType::FF),
                '&' => (raw[1..].to_string(), ModuleType::CJ),
                _ => panic!(),
            }
        };

        Self {
            name,
            mtype,
            on: false,
            inputs: vec![],
            outputs,
        }
    }
}

fn init_inputs(modules: &mut HashMap<String, Module>) {
    let conjunctions: Vec<String> = modules
        .iter()
        .filter_map(|(n, m)| match m.mtype {
            ModuleType::CJ => Some(n.clone()),
            _ => None,
        })
        .collect();

    // this needs some refactors...
    let mut aaa: HashMap<String, Vec<String>> = HashMap::new();
    for s in &conjunctions {
        aaa.insert(s.clone(), vec![]);
    }

    for (n, m) in modules.iter_mut() {
        for n2 in &m.outputs {
            if conjunctions.contains(n2) {
                // insert n into n2.inputs
                let v = aaa.get_mut(n2).unwrap();
                v.push(n.clone());
            }
        }
    }

    for (cjn, inn) in aaa {
        let module = modules.get_mut(&cjn).unwrap();
        for i in 0..inn.len() {
            module.inputs.push((inn[i].clone(), Pulse::Low));
        }
    }
}

type Operation = (String, Pulse, String);

fn get_into_rx(modules: &HashMap<String, Module>) -> String {
    for (n, m) in modules {
        if m.outputs.contains(&"rx".to_string()) {
            return n.clone();
        }
    }
    panic!();
}

fn get_into_pre_rx(pre_rx: &str, modules: &HashMap<String, Module>) -> Vec<String> {
    let mut into_pre_rx: Vec<String> = vec![];
    for (n, m) in modules {
        if m.outputs.contains(&pre_rx.to_string()) {
            into_pre_rx.push(n.clone());
        }
    }
    into_pre_rx
}

fn p2(lines: Vec<String>) {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in lines {
        let module = Module::new(&line);
        modules.insert(module.name.clone(), module);
    }
    init_inputs(&mut modules);

    let mut ops: VecDeque<Operation> = VecDeque::new();
    let bc_out = {
        modules
            .get(&"broadcaster".to_string())
            .unwrap()
            .outputs
            .clone()
    };

    let into_rx = get_into_rx(&modules);
    let into_pre_rx = get_into_pre_rx(&into_rx, &modules);

    let mut cycle_lengths: HashMap<String, Option<i32>> = HashMap::new();
    for s in &into_pre_rx {
        cycle_lengths.insert(s.clone(), None);
    }

    // For each INTO_PRE_RX
    // As soon as it sends a high pulse to INTO_RX
    // Store its cycle length
    // LCM the cycle lengths

    for i in 0..4096 {
        for out in &bc_out {
            ops.push_back(("broadcaster".to_string(), Pulse::Low, out.clone()));
        }

        while !ops.is_empty() {
            let op = ops.pop_front().unwrap();
            let pulse = op.1;

            let sender_name = {
                let sender = modules.get(&op.0).unwrap();
                sender.name.clone()
            };
            let receiver_opt = modules.get_mut(&op.2); //.unwrap();
            let receiver = match receiver_opt {
                Some(r) => r,
                None => continue,
            };

            if receiver.name == into_rx && pulse == Pulse::High {
                let presses = cycle_lengths.get(&receiver.name);
                match presses {
                    Some(_) => (),
                    None => {
                        cycle_lengths.insert(sender_name.clone(), Some(i + 1));

                        if cycle_lengths.values().all(|v| v.is_some()) {
                            fn gcd(a: i64, b: i64) -> i64 {
                                match b {
                                    0 => a,
                                    _ => gcd(b, a % b),
                                }
                            }

                            fn lcm(a: i64, b: i64) -> i64 {
                                a / gcd(a, b) * b
                            }

                            fn lcmm(nums: &Vec<i64>) -> i64 {
                                nums.iter().fold(1, |acc, &e| lcm(acc, e))
                            }

                            let nums: Vec<i64> =
                                cycle_lengths.values().map(|v| v.unwrap() as i64).collect();

                            println!("Part 2: {}", lcmm(&nums));
                        }
                    }
                }
            }

            match receiver.mtype {
                ModuleType::FF => match pulse {
                    Pulse::Low => {
                        receiver.on = !receiver.on;

                        let puls = {
                            match receiver.on {
                                true => Pulse::High,
                                false => Pulse::Low,
                            }
                        };

                        for output in &receiver.outputs {
                            ops.push_back((receiver.name.clone(), puls.clone(), output.clone()));
                        }
                    }
                    Pulse::High => (),
                },
                ModuleType::CJ => {
                    for input in receiver.inputs.iter_mut() {
                        if input.0 == sender_name {
                            input.1 = pulse.clone();
                        }
                    }

                    let puls = {
                        let all_high = receiver.inputs.iter().all(|i| i.1 == Pulse::High);
                        match all_high {
                            true => Pulse::Low,
                            false => Pulse::High,
                        }
                    };

                    for output in &receiver.outputs {
                        ops.push_back((receiver.name.clone(), puls.clone(), output.clone()));
                    }
                }
                _ => panic!(),
            }
        }
    }
}

fn p1(lines: Vec<String>) {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in lines {
        let module = Module::new(&line);
        modules.insert(module.name.clone(), module);
    }
    init_inputs(&mut modules);

    let mut ops: VecDeque<Operation> = VecDeque::new();
    let bc_out = {
        modules
            .get(&"broadcaster".to_string())
            .unwrap()
            .outputs
            .clone()
    };

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        total_low += 1;
        for out in &bc_out {
            ops.push_back(("broadcaster".to_string(), Pulse::Low, out.clone()));
        }

        while !ops.is_empty() {
            let op = ops.pop_front().unwrap();
            let pulse = op.1;
            match pulse {
                Pulse::High => total_high += 1,
                Pulse::Low => total_low += 1,
            }

            let sender_name = {
                let sender = modules.get(&op.0).unwrap();
                sender.name.clone()
            };
            let receiver_opt = modules.get_mut(&op.2); //.unwrap();
            let receiver = match receiver_opt {
                Some(r) => r,
                None => continue,
            };

            match receiver.mtype {
                ModuleType::FF => match pulse {
                    Pulse::Low => {
                        receiver.on = !receiver.on;

                        let puls = {
                            match receiver.on {
                                true => Pulse::High,
                                false => Pulse::Low,
                            }
                        };

                        for output in &receiver.outputs {
                            ops.push_back((receiver.name.clone(), puls.clone(), output.clone()));
                        }
                    }
                    Pulse::High => (),
                },
                ModuleType::CJ => {
                    for input in receiver.inputs.iter_mut() {
                        if input.0 == sender_name {
                            input.1 = pulse.clone();
                        }
                    }

                    let puls = {
                        let all_high = receiver.inputs.iter().all(|i| i.1 == Pulse::High);
                        match all_high {
                            true => Pulse::Low,
                            false => Pulse::High,
                        }
                    };

                    for output in &receiver.outputs {
                        ops.push_back((receiver.name.clone(), puls.clone(), output.clone()));
                    }
                }
                _ => panic!(),
            }
        }
    }

    println!("{} {}", total_low, total_high);
    println!("Part 1: {}", total_low * total_high);
}

pub fn d20() {
    let input_file = Path::new("src/d20/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
