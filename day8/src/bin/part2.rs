use num::Integer;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();

    let map: HashMap<_, _> = lines
        .map(|line| {
            let parsed = line.replace(['=','(',',',')'], "");
            let mut parsed = parsed.split_whitespace();
            let element = parsed.next().unwrap().to_string();
            let left = parsed.next().unwrap().to_string();
            let right = parsed.next().unwrap().to_string();
            (element, (left, right))
        })
        .collect();

    map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|key| follow_map(instructions, key, &map))
        .reduce(|a, b| a.lcm(&b)).unwrap()
}

fn follow_map(instructions: &str, start: &str, map: &HashMap<String, (String, String)>) -> usize {
    let mut count = 0;
    let mut current = start;
    for direction in instructions.chars().cycle() {
        let node = map.get(current).unwrap();
        match direction {
            'L' => current = &node.0,
            'R' => current = &node.1,
            _ => unreachable!(),
        }

        count += 1;

        if current.ends_with('Z') {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample2.txt")), 6);
    }
}
