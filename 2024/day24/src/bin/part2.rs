use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use common::{Context, Itertools};

/**
 * This is not a general solver or solution, rather it outputs the wires (which represent a ripple
 * adder) in a format where you can see the patterns. You can then hardcode the swaps until
 * everything matches and all the wires used, and the output matches the expected sum of x and y
 */
fn main() {
    let input: String = common::AocInput::fetch(2024, 24).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum GateKind {
    And,
    Xor,
    Or,
}

impl Display for GateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Or => "OR",
                Self::Xor => "XOR",
                Self::And => "AND",
            }
        )
    }
}

const NUM_BITS: usize = 45;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    kind: GateKind,
    output: &'a str,
}

impl Display for Gate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.a, self.kind, self.b, self.output)
    }
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

fn do_swaps(s: &str) -> &str {
    let swaps = [
        ("fbq", "z36"),
        ("qqp", "z23"),
        ("z16", "pbv"),
        ("qff", "qnw"),
    ];

    for (a, b) in swaps {
        if s == a {
            println!("swapped {s} with {b}");
            return b;
        } else if s == b {
            println!("swapped {s} with {a}");
            return a;
        }
    }
    s
}

fn process(input: &str) -> String {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let mut wires: HashMap<_, _> = wires
        .lines()
        .map(|l| {
            let (wire, value) = l.split_once(": ").unwrap();
            (wire, value.parse::<u8>().unwrap())
        })
        .collect();
    let x = extract_number('x', &wires);
    let y = extract_number('y', &wires);

    dbg!((x, y));

    let mut gates: VecDeque<_> = gates
        .lines()
        .sorted()
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let (a, kind, b) = input.splitn(3, " ").collect_tuple().unwrap();
            let mut ab = [a, b];
            ab.sort();
            let output = do_swaps(output);

            let kind = match kind {
                "AND" => GateKind::And,
                "XOR" => GateKind::Xor,
                "OR" => GateKind::Or,
                _ => panic!("Unknwon OP"),
            };
            Gate { a, b, kind, output }
        })
        .collect();

    let mut found = HashSet::new();
    for i in 0..NUM_BITS {
        match validate_half_adders(&gates, i, &mut found) {
            Ok(_) => {}
            Err(e) => {
                if i > 0 {
                    println!("\nISSUE FOUND: {e}")
                } else {
                    println!();
                }
            }
        }
    }

    let gate_set = HashSet::from_iter(gates.iter());
    let diff = gate_set.difference(&found);

    println!("\n\nRemaining wires:");
    dbg!(diff.collect_vec());
    while let Some(gate) = gates.pop_front() {
        if gate.evaluate(&mut wires).is_none() {
            gates.push_back(gate);
        }
    }
    let output = extract_number('z', &wires);

    println!("\nexpect: {:b}", x + y);
    println!("output: {output:b}");

    "fbq,pbv.qff.qnw,qqp,z16.z23,z36".to_string()
}

fn validate_half_adders<'a>(
    gates: &'a VecDeque<Gate<'a>>,
    i: usize,
    found: &mut HashSet<&Gate<'a>>,
) -> common::Result<()> {
    let x = format!("x{i:02}");
    let y = format!("y{i:02}");
    let sum1 = gates
        .iter()
        .find(|g| g.kind == GateKind::Xor && ((g.a == x && g.b == y) || (g.a == y && g.b == x)))
        .context("input {x} XOR {y}")?;

    print!("{} | ", sum1);
    found.insert(sum1);

    let carry1 = gates
        .iter()
        .find(|g| g.kind == GateKind::And && ((g.a == x && g.b == y) || (g.a == y && g.b == x)))
        .context(format!("{i}: carry 1 {x} AND {y}"))?;
    common::ensure!(
        !carry1.output.starts_with("z"),
        "{i}: carry1 should not output to z {carry1:?}"
    );

    print!("{} | ", carry1);
    found.insert(carry1);

    let sum2 = gates
        .iter()
        .find(|g| g.kind == GateKind::Xor && (g.a == sum1.output || g.b == sum1.output))
        .context(format!(
            "{i}: sum2, expexted {} XOR <something>",
            sum1.output
        ))?;

    print!("{} | ", sum2);
    common::ensure!(
        sum2.output.starts_with("z"),
        "{i}: sum2 should output to z {sum2:?}"
    );

    found.insert(sum2);

    let carry2 = gates
        .iter()
        .find(|g| {
            g.kind == GateKind::And
                && ((g.a == sum2.a && g.b == sum2.b) || (g.a == sum2.b && g.b == sum2.a))
        })
        .context(format!("{i}: carry 2 expected {} AND {}", sum2.a, sum2.b))?;

    print!("{} | ", carry2);
    common::ensure!(
        !carry2.output.starts_with("z"),
        "{i}: carry1 should not output to z {carry2:?}"
    );

    found.insert(carry2);
    let carry_out = gates
        .iter()
        .find(|g| {
            g.kind == GateKind::Or
                && ((g.a == carry1.output && g.b == carry2.output)
                    || (g.a == carry2.output && g.b == carry1.output))
        })
        .context(format!(
            "{i}: output carry expected {} AND {}",
            carry1.output, carry2.output
        ))?;

    println!("{}", carry_out);
    common::ensure!(
        !carry_out.a.starts_with("z"),
        "{i}: carry_out.a should not be a z {carry_out:?}"
    );
    common::ensure!(
        !carry_out.b.starts_with("z"),
        "{i}: carry_out.b should not be a z {carry_out:?}"
    );
    common::ensure!(
        !carry_out.a.starts_with("x"),
        "{i}: carry_out.a should not be a x {carry_out:?}"
    );
    common::ensure!(
        !carry_out.b.starts_with("x"),
        "{i}: carry_out.b should not be a x {carry_out:?}"
    );
    common::ensure!(
        !carry_out.a.starts_with("y"),
        "{i}: carry_out.a should not be a y {carry_out:?}"
    );
    common::ensure!(
        !carry_out.b.starts_with("y"),
        "{i}: carry_out.b should not be a y {carry_out:?}"
    );
    found.insert(carry_out);
    Ok(())
}

fn extract_number(prefix: char, wires: &HashMap<&str, u8>) -> usize {
    wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted()
        .enumerate()
        .map(|(i, (_, bit))| *bit as usize * 2usize.pow(i as u32))
        .sum()
}
