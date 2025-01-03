use std::collections::HashMap;

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 19).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (patterns, designs) = input.split_once("\n\n").expect("valid input");

    let patterns = patterns.split(", ").collect_vec();
    let mut cache = HashMap::new();
    designs
        .lines()
        .map(|design| count_possibilities(design, &patterns, &mut cache))
        .sum()
}

fn count_possibilities<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            if cache.contains_key(&(design, *pattern)) {
                return *cache.get(&(design, *pattern)).unwrap();
            }

            if let Some(stripped) = design.strip_prefix(pattern) {
                if stripped.is_empty() {
                    return 1;
                }
                let possible = count_possibilities(stripped, patterns, cache);
                cache.insert((design, pattern), possible);
                return possible;
            }

            cache.insert((design, pattern), 0);
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 16);
    }
}
