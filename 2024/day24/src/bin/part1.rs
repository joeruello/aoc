use std::collections::{HashMap, VecDeque};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 24).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
enum GateKind {
    And,
    Xor,
    Or,
}

#[derive(Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    kind: GateKind,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn evaluate(&self, wires: &mut HashMap<&'a str, u8>) -> Option<()> {
        let a = wires.get(self.a)?;
        let b = wires.get(self.b)?;

        wires.insert(
            self.output,
            match self.kind {
                GateKind::And => a & b,
                GateKind::Xor => a ^ b,
                GateKind::Or => a | b,
            },
        );

        Some(())
    }
}

fn process(input: &str) -> usize {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let mut wires: HashMap<_, _> = wires
        .lines()
        .map(|l| {
            let (wire, value) = l.split_once(": ").unwrap();
            (wire, value.parse::<u8>().unwrap())
        })
        .collect();

    let mut gates: VecDeque<_> = gates
        .lines()
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let (a, kind, b) = input.splitn(3, " ").collect_tuple().unwrap();
            let kind = match kind {
                "AND" => GateKind::And,
                "XOR" => GateKind::Xor,
                "OR" => GateKind::Or,
                _ => panic!("Unknwon OP"),
            };
            Gate { a, b, kind, output }
        })
        .collect();

    while let Some(gate) = gates.pop_front() {
        if gate.evaluate(&mut wires).is_none() {
            gates.push_back(gate);
        }
    }

    extract_output(wires)
}

fn extract_output(wires: HashMap<&str, u8>) -> usize {
    let mut z_bits = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect_vec();
    z_bits.sort();

    let mut accum = 0;
    for (i, (_, bit)) in z_bits.into_iter().enumerate() {
        accum += bit as usize * 2usize.pow(i as u32);
    }
    accum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 4);
        assert_eq!(process(include_str!("./sample2.txt")), 2024);
    }
}
