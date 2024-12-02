use std::collections::HashMap;

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("   ").unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            (a, b)
        })
        .collect();

    let right = right.into_iter().counts();

    left.into_iter()
        .map(|a| {
            let freq = right.get(&a).unwrap_or(&0);
            freq * a
        })
        .sum()
}
