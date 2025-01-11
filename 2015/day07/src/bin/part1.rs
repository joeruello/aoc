use std::collections::{HashMap, VecDeque};

fn main() {
    let input: String = common::AocInput::fetch(2015, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug, Clone, Hash, PartialEq)]
enum Node {
    Eq(Input),
    Not(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, Input),
    RShift(Input, Input),
}

#[derive(Debug, Clone, Hash, PartialEq)]
enum Input {
    Const(u16),
    Wire(String),
}

fn parse_input(input: &str) -> Input {
    input
        .parse::<u16>()
        .map_or(Input::Wire(input.to_string()), Input::Const)
}

fn process(input: &str) -> usize {
    let nodes: VecDeque<(String, Node)> = input
        .lines()
        .map(|l| {
            let (op, wire) = l.split_once(" -> ").expect("Should have arrow");
            let wire = wire.to_string();
            let mut op: Vec<_> = op.split_whitespace().collect();

            match op.len() {
                1 => {
                    let input = parse_input(op.pop().unwrap());
                    (wire, Node::Eq(input))
                }
                2 => {
                    let input = parse_input(op.last().unwrap());
                    (wire, Node::Not(input))
                }
                3 => {
                    let a = parse_input(op.first().unwrap());
                    let b = parse_input(op.last().unwrap());
                    (
                        wire,
                        match *op.get(1).unwrap() {
                            "AND" => Node::And(a, b),
                            "OR" => Node::Or(a, b),
                            "LSHIFT" => Node::LShift(a, b),
                            "RSHIFT" => Node::RShift(a, b),
                            _ => unreachable!(),
                        },
                    )
                }
                _ => unreachable!(),
            }
        })
        .collect();
    let mut circuit = Circuit::new();

    simulate(nodes, &mut circuit);

    circuit
        .get_input_signal(&Input::Wire("a".to_string()))
        .unwrap() as usize
}

fn simulate(mut nodes: VecDeque<(String, Node)>, circuit: &mut Circuit) {
    while let Some((wire, node)) = nodes.pop_front() {
        match &node {
            Node::Eq(input) => {
                if let Some(signal) = circuit.get_input_signal(input) {
                    circuit.signals.insert(wire, signal);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
            Node::Not(input) => {
                if let Some(signal) = circuit.get_input_signal(input) {
                    circuit.signals.insert(wire, !signal);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
            Node::And(a, b) => {
                if let Some((a, b)) = circuit.get_input_signals(a, b) {
                    circuit.signals.insert(wire, a & b);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
            Node::Or(a, b) => {
                if let Some((a, b)) = circuit.get_input_signals(a, b) {
                    circuit.signals.insert(wire, a | b);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
            Node::LShift(a, b) => {
                if let Some((a, b)) = circuit.get_input_signals(a, b) {
                    circuit.signals.insert(wire, a << b);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
            Node::RShift(a, b) => {
                if let Some((a, b)) = circuit.get_input_signals(a, b) {
                    circuit.signals.insert(wire, a >> b);
                } else {
                    nodes.push_back((wire.to_owned(), node));
                };
            }
        }
    }
}

struct Circuit {
    pub signals: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn get_input_signal(&self, input: &Input) -> Option<u16> {
        match &input {
            Input::Const(c) => Some(*c),
            Input::Wire(a) => self.signals.get(a).copied(),
        }
    }

    pub fn get_input_signals(&self, a: &Input, b: &Input) -> Option<(u16, u16)> {
        let a = self.get_input_signal(a)?;
        let b = self.get_input_signal(b)?;
        Some((a, b))
    }
}
