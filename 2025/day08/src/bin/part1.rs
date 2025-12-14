use std::collections::HashMap;

use common::Itertools;
use glam::I64Vec3;

fn main() {
    let input: String = common::AocInput::fetch(2025, 8).unwrap().into();
    println!("Output: {}", process(&input, 1000));
}

fn process(input: &str, connections: usize) -> usize {
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
        .sorted_by_key(|(_, d)| *d)
        .take(connections);

    let mut circuits: HashMap<I64Vec3, usize> = HashMap::new();

    for (idx, point) in points.iter().enumerate() {
        circuits.insert(*point, idx);
    }

    for ((a, b), _) in distances {
        let cir_a = *circuits.get(a).unwrap();
        let cir_b = *circuits.get(b).unwrap();

        circuits = circuits
            .into_iter()
            .map(|(k, v)| if v == cir_b { (k, cir_a) } else { (k, v) })
            .collect()
    }

    let mut inverted = HashMap::new();

    for (k, v) in circuits {
        inverted.entry(v).or_insert(Vec::new()).push(k);
    }

    inverted
        .values()
        .map(|v| v.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 10), 40);
    }
}
