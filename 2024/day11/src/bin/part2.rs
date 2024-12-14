use std::collections::HashMap;

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 11).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .counts();

    for n in 0..75 {
        stones = step(stones);
        println!("{n}:{}", stones.len());
    }

    stones.values().sum()
}

fn step(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut ret = HashMap::new();

    for (n, c) in stones {
        let digits = n.checked_ilog10().unwrap_or(0) + 1;
        if n == 0 {
            *ret.entry(1).or_default() += c;
        } else if digits % 2 == 0 {
            let bottom = n % 10u64.pow(digits / 2);
            let top = n / 10u64.pow(digits / 2);
            *ret.entry(top).or_default() += c;

            *ret.entry(bottom).or_default() += c;
        } else {
            *ret.entry(n * 2024).or_default() += c;
        }
    }
    ret
}
