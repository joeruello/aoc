use std::collections::{HashMap, HashSet};

use common::Itertools;
use glam::I64Vec3;

fn main() {
    let input: String = common::AocInput::fetch(2025, 8).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i64 {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            I64Vec3 { x, y, z }
        })
        .collect();

    let distances = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((a, b), a.distance_squared(*b)))
        .sorted_by_key(|(_, d)| *d);

    let mut circuits: HashMap<I64Vec3, usize> = HashMap::new();

    for (idx, point) in points.iter().enumerate() {
        circuits.insert(*point, idx);
    }

    for ((a, b), _) in distances {
        let cir_a = *circuits.get(a).unwrap();
        let cir_b = *circuits.get(b).unwrap();
        let mut cirs = HashSet::new();

        circuits = circuits
            .into_iter()
            .map(|(k, v)| {
                let cir = { if v == cir_b { cir_a } else { v } };
                cirs.insert(cir);
                (k, cir)
            })
            .collect();

        if cirs.len() == 1 {
            return a.x * b.x;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 25272);
    }
}
