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

struct Circuit {
    pub signals: HashMap<String, u16>,
}

fn process(input: &str) -> usize {
    let instructions: VecDeque<(String, Node)> =
        input.lines().map(|l| parse_line(l).unwrap()).collect();

    let mut circuit = Circuit::new();
    circuit.run(instructions.clone());

    let a_result = circuit.get_signal(&"a".into()).unwrap();

    let mut circuit = Circuit::new();
    let mut instructions: VecDeque<_> = instructions
        .into_iter()
        .filter(|(wire, _)| wire != "b")
        .collect();

    instructions.push_front(("b".to_string(), Node::Eq(Input::Const(a_result))));

    circuit.run(instructions);

    circuit.get_signal(&"a".into()).unwrap() as usize
}

impl Circuit {
    fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn run(&mut self, mut nodes: VecDeque<(String, Node)>) {
        while let Some((wire, node)) = nodes.pop_front() {
            match &node {
                Node::Eq(input) => {
                    if let Some(signal) = self.get_signal(input) {
                        self.signals.insert(wire, signal);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
                Node::Not(input) => {
                    if let Some(signal) = self.get_signal(input) {
                        self.signals.insert(wire, !signal);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
                Node::And(a, b) => {
                    if let Some((a, b)) = self.get_signals(a, b) {
                        self.signals.insert(wire, a & b);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
                Node::Or(a, b) => {
                    if let Some((a, b)) = self.get_signals(a, b) {
                        self.signals.insert(wire, a | b);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
                Node::LShift(a, b) => {
                    if let Some((a, b)) = self.get_signals(a, b) {
                        self.signals.insert(wire, a << b);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
                Node::RShift(a, b) => {
                    if let Some((a, b)) = self.get_signals(a, b) {
                        self.signals.insert(wire, a >> b);
                    } else {
                        nodes.push_back((wire.to_owned(), node));
                    };
                }
            }
        }
    }

    pub fn get_signal(&self, input: &Input) -> Option<u16> {
        match &input {
            Input::Const(c) => Some(*c),
            Input::Wire(a) => self.signals.get(a).copied(),
        }
    }

    pub fn get_signals(&self, a: &Input, b: &Input) -> Option<(u16, u16)> {
        let a = self.get_signal(a)?;
        let b = self.get_signal(b)?;
        Some((a, b))
    }
}

fn parse_input(input: &str) -> Input {
    input.parse::<u16>().map_or(input.into(), Input::Const)
}

fn parse_line(l: &str) -> Option<(String, Node)> {
    let (op, wire) = l.split_once(" -> ").expect("Should have arrow");
    let wire = wire.to_string();
    let mut op: Vec<_> = op.split_whitespace().collect();

    match op.len() {
        1 => {
            let input = parse_input(op.pop()?);
            Some((wire, Node::Eq(input)))
        }
        2 => {
            let input = parse_input(op.last()?);
            Some((wire, Node::Not(input)))
        }
        3 => {
            let a = parse_input(op.first()?);
            let b = parse_input(op.last()?);
            Some((
                wire,
                match *op.get(1)? {
                    "AND" => Node::And(a, b),
                    "OR" => Node::Or(a, b),
                    "LSHIFT" => Node::LShift(a, b),
                    "RSHIFT" => Node::RShift(a, b),
                    _ => unreachable!(),
                },
            ))
        }
        _ => None,
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self::Wire(value.to_string())
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::Wire(value)
    }
}

impl From<u16> for Input {
    fn from(value: u16) -> Self {
        Self::Const(value)
    }
}
