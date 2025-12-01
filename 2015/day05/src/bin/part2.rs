use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2015, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l)).count()
}

fn is_nice(line: &str) -> bool {
    let mut pairs = HashMap::<char, HashSet<usize>>::new();
    let mut repeats = HashSet::<usize>::new();
    for (i, (a, b, c)) in line.chars().tuple_windows().enumerate() {
        if a == b {
            pairs
                .entry(a)
                .and_modify(|s| {
                    if !s.contains(&(i - 1)) {
                        s.insert(i);
                    }
                })
                .or_insert(HashSet::from([i]));

            println!("Found pair {a} {b} at {i}");
        }
        if a == c {
            println!("Found repeat {a} {c}");
            repeats.insert(i);
        }
    }

    dbg!(&pairs);
    dbg!(&repeats);
    pairs.into_values().filter(|p| p.len() > 1).count() > 0 && !repeats.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!is_nice("aaa"));
        assert!(is_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_nice("xxyxx"));
        assert!(!is_nice("uurcxstgmygtbstg"));
        assert!(!is_nice("ieodomkazucvgmuy"));
    }
}
