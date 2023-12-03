use std::cmp::min;

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    let schematic: Vec<_> = input.lines().map(|l| l.trim()).collect();
    let numbers_pattern = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;

    for (y, line) in schematic.iter().enumerate() {
        for mat in numbers_pattern.find_iter(line) {
            for x in mat.range() {
                if is_adjacent_to_symbol(&schematic, x, y, line.len(), schematic.len()) {
                    sum += mat
                        .as_str()
                        .parse::<u32>()
                        .expect("Num should be a valid integer");
                    break;
                }
            }
        }
    }

    sum
}

fn is_adjacent_to_symbol(arr: &[&str], x0: usize, y0: usize, x_max: usize, y_max: usize) -> bool {
    for y in y0.saturating_sub(1)..=min(y0 + 1, y_max) {
        for x in x0.saturating_sub(1)..=min(x0 + 1, x_max) {
            if x == x0 && y == y0 {
                continue;
            }
            if arr
                .get(y)
                .and_then(|y| y.chars().nth(x))
                .is_some_and(|c| !c.is_numeric() && c != '.')
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 4361);
    }

}
