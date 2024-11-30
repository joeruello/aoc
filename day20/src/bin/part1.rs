use std::{
    collections::{HashMap, VecDeque},
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop { on: bool },
    Conjuction { memory: HashMap<String, Pulse> },
    Broadcast,
}

impl Module {
    fn flip(&mut self) {
        match self {
            Module::FlipFlop { on } => {
                match on {
                    true => *on = false,
                    false => *on = true,
                }
            }
            _ => panic!("Can only flip a flipflop"),
        }
    }
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(input, 1000));
}

fn process(input: &str, button_presses: usize) -> usize {
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
            "%" => modules.insert(module_name.to_string(), Module::FlipFlop { on: false }),
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
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _ in 0..button_presses {
        queue.push_back((Pulse::Low, "broadcaster", "button"));
        while let Some((pulse, key, source)) = queue.pop_front() {
            // println!("{source} -{pulse:?}-> {key}");
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }
            let module = modules.get_mut(key);
            let destinations = outputs.get(key).unwrap_or(&empty_vec);
            match module {
                Some(Module::Broadcast) => {
                    for dest in destinations {
                        queue.push_back((pulse, dest, key))
                    }
                }
                Some(Module::FlipFlop { on }) => match (pulse, on) {
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
                },
                None => {}
            }
        }
    }
dbg!(high_pulses, low_pulses);
    high_pulses * low_pulses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_once_press() {
        assert_eq!(process(include_str!("./sample.txt"), 1), 32);
    }

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt"), 1000), 32000000);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(process(include_str!("./sample2.txt"), 1000), 11687500);
    }
}
