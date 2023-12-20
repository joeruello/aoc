use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlip { on: bool },
    Conjuction { memory: HashMap<String, Pulse> },
    Broadcast,
}

impl Module {
    fn flip(&mut self) {
        match self {
            Module::FlipFlip { on } => match on {
                true => *on = false,
                false => *on = true,
            },
            _ => panic!("Can only flip a flipflop"),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, "rx".to_string()));
}

fn process(input: &str, target: String) -> usize {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut outputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let (module, destinations) = line.split_once(" -> ").unwrap();
        let (module_type, mut module_name) = module.split_at(1);
        match module_type {
            "b" => {
                module_name = "broadcaster";
                modules.insert("broadcaster".to_string(), Module::Broadcast)
            }
            "%" => modules.insert(module_name.to_string(), Module::FlipFlip { on: false }),
            "&" => modules.insert(
                module_name.to_string(),
                Module::Conjuction {
                    memory: Default::default(),
                },
            ),
            _ => panic!("Unknown module type {module_type}"),
        };

        let destinations: Vec<String> = destinations.split(", ").map(|s| s.to_string()).collect();

        for destintation in destinations.iter() {
            inputs
                .entry(destintation.to_string())
                .and_modify(|v| v.push(module_name.to_string()))
                .or_insert(vec![module_name.to_string()]);
        }

        outputs.insert(module_name.to_string(), destinations);
    }

    // Setup conjection memory
    for (name, module) in modules.iter_mut() {
        if let Module::Conjuction { memory } = module {
            for input in inputs.get(name).unwrap() {
                memory.insert(input.to_string(), Pulse::Low);
            }
        }
    }

    let empty_vec = vec![];
    let mut queue = VecDeque::new();

    let mut parent_cycle_counts: HashMap<String, usize> = HashMap::new();
    
    // Assumed to be 1 by inspection of inputs
    let target_input = inputs.get(&target).unwrap().first().unwrap();
    let target_input_inputs = inputs.get(target_input).unwrap();

    for num_presses in 1.. {
        queue.push_back((Pulse::Low, "broadcaster", "button"));
        while let Some((pulse, key, source)) = queue.pop_front() {
            if pulse == Pulse::High && key == "mf" {
                parent_cycle_counts.entry(source.to_owned()).or_insert(num_presses);
            }

            let module = modules.get_mut(key);
            let destinations = outputs.get(key).unwrap_or(&empty_vec);
            match module {
                Some(Module::Broadcast) => {
                    for dest in destinations {
                        queue.push_back((pulse, dest, key))
                    }
                }
                Some(Module::FlipFlip { on }) => match (pulse, on) {
                    (Pulse::Low, true) => {
                        for dest in destinations {
                            queue.push_back((Pulse::Low, dest, key))
                        }
                        module.unwrap().flip();
                    }
                    (Pulse::Low, false) => {
                        for dest in destinations {
                            queue.push_back((Pulse::High, dest, key))
                        }
                        module.unwrap().flip();
                    }
                    (Pulse::High, _) => {}
                },
                Some(Module::Conjuction { memory }) => {
                    memory.entry(source.to_string()).and_modify(|p| *p = pulse);

                    let output = if memory.values().all(|p| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for dest in destinations {
                        queue.push_back((output, dest, key))
                    }
                }
                None => {} // "Output" modules
            }
        }
        if parent_cycle_counts.len() == target_input_inputs.len() {
            dbg!(&parent_cycle_counts);
            break;
        }
    }

    parent_cycle_counts
        .values()
        .map(|u| u.to_owned())
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}
