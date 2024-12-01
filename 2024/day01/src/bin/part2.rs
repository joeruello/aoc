use std::collections::HashMap;

fn main() {
    let input: String = common::AocInput::fetch(2024, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let pairs: Vec<_> = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("   ").unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            (a, b)
        })
        .collect();

    let mut left = vec![];
    let mut right = HashMap::<usize, usize>::new();

    for (a, b) in pairs {
        left.push(a);
        right.entry(b).and_modify(|f| *f += 1).or_insert(1);
    }

    left.sort();

    left.into_iter()
        .map(|a| {
            let freq = right.get(&a).unwrap_or(&0);
            freq * a
        })
        .sum()
}
