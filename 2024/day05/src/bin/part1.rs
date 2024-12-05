use std::{cmp::Ordering, collections::HashSet};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").expect("two parts");
    let ordering_rules: HashSet<_> = rules
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").expect("delimited");

            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .filter(|u| u.is_sorted_by(|a, b| compare(*a, *b, &ordering_rules) != Ordering::Greater))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn compare(a: usize, b: usize, rules: &HashSet<(usize, usize)>) -> Ordering {
    match rules.get(&(a, b)) {
        Some(_) => Ordering::Less,
        None => match rules.get(&(b, a)) {
            Some(_) => Ordering::Greater,
            None => Ordering::Equal,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process(include_str!("./sample.txt")), 143);
    }
}
