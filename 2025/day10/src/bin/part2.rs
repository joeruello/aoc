//https://aoc.winslowjosiah.com/solutions/2025/day/10/
//
use std::{cmp::min, collections::HashMap, ops::Deref, usize};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 10).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
struct Machine {
    pub target: Vec<u16>,
    pub buttons: Vec<Button>,
}
#[derive(Debug, Clone)]
struct Button(Vec<u16>);

impl Button {
    fn new(vec: Vec<u16>) -> Self {
        Self(vec)
    }

    fn mask(&self) -> u16 {
        self.0.iter().map(|d| 2u16.pow(*d as u32)).sum()
    }
}

impl Deref for Button {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let _lights = parts.next().unwrap();
            let joltage = parts
                .next_back()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .filter_map(|c| c.parse().ok())
                .collect();
            let buttons = parts
                .map(|c| {
                    Button::new(
                        c.chars()
                            .filter_map(|c| c.to_digit(10).map(|d| d as u16))
                            .collect_vec(),
                    )
                })
                .collect();

            Machine {
                target: joltage,
                buttons,
            }
        })
        .map(|m| {
            let patterns = find_all_valid_patterns(&m);
            let mut cache = HashMap::new();
            let presses = get_min_presses(m.target, &patterns, &mut cache);
            presses.unwrap()
        })
        .sum()
}

fn find_all_valid_patterns(machine: &Machine) -> HashMap<u16, Vec<Vec<Button>>> {
    let mut patterns = HashMap::new();
    for num_presses in 0..=machine.buttons.len() {
        for presses in machine.buttons.iter().combinations(num_presses) {
            let mut pattern: u16 = 0;
            let mut pressed = vec![];
            for btn in presses {
                pattern ^= btn.mask();
                pressed.push(btn.to_owned());
            }
            patterns.entry(pattern).or_insert(vec![]).push(pressed);
        }
    }

    patterns
}

fn get_min_presses(
    target: Vec<u16>,
    patterns: &HashMap<u16, Vec<Vec<Button>>>,
    cache: &mut HashMap<Vec<u16>, Option<usize>>,
) -> Option<usize> {
    if target.iter().all(|d| *d == 0) {
        return Some(0);
    }

    if let Some(presses) = cache.get(&target) {
        return *presses;
    }

    let mut odd_mask: u16 = 0;
    for (i, c) in target.iter().enumerate() {
        if c % 2 == 1 {
            odd_mask += 1 << i;
        }
    }

    let mut result = None;

    if patterns.get(&odd_mask).is_some() {
        'outer: for presses in patterns.get(&odd_mask).unwrap() {
            let mut after = target.clone();
            for btn in presses.iter() {
                for idx in btn.iter() {
                    if after[*idx as usize] == 0 {
                        continue 'outer;
                    }
                    after[*idx as usize] -= 1;
                }
            }

            let halved = after.iter().map(|d| d / 2).collect_vec();

            if let Some(halved_presses) = get_min_presses(halved, patterns, cache) {
                let num_presses = presses.len() + 2 * halved_presses;

                result = Some(match result {
                    Some(prev) => min(prev, num_presses),
                    None => num_presses,
                })
            }
        }
    }

    cache.insert(target, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 33);
    }
}
