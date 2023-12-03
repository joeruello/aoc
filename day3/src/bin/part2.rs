use std::{cmp::min, collections::HashMap};

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    let schematic: Vec<_> = input.lines().map(|l| l.trim()).collect();
    let numbers_pattern = Regex::new(r"(\d+)").unwrap();
    let mut gears = HashMap::new();

    for (y, line) in schematic.iter().enumerate() {
        for m in numbers_pattern.find_iter(line) {
            for x in m.range() {
                if let Some(gear_coords) = find_adjacent_gear(&schematic, x, y, line.len(), schematic.len()) {
                    gears.entry(gear_coords).or_insert(vec![]).push(
                        m.as_str()
                            .parse::<u32>()
                            .expect("Num should be a valid integer"),
                    );
                    break;
                }
            }
        }
    }

    gears
        .into_iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums.into_iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn find_adjacent_gear(
    arr: &[&str],
    x0: usize,
    y0: usize,
    x_max: usize,
    y_max: usize,
) -> Option<(usize, usize)> {
    for y in y0.saturating_sub(1)..=min(y0 + 1, y_max) {
        for x in x0.saturating_sub(1)..=min(x0 + 1, x_max) {
            if x == x0 && y == y0 {
                continue;
            }
            if arr
                .get(y)
                .and_then(|y| y.chars().nth(x))
                .is_some_and(|c| c == '*')
            {
                return Some((x, y));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 467835);
    }
}
