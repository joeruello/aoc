use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let _ = lines.next();

    let map: HashMap<_, _> = lines
        .map(|line| {
            let parsed = line.replace(" = (", " ").replace(',', "").replace(")", "");
            let mut parsed = parsed.split_whitespace();
            let element = parsed.next().unwrap().to_string();
            let left = parsed.next().unwrap().to_string();
            let right = parsed.next().unwrap().to_string();
            (element, (left, right))
        })
        .collect();

    let mut current = "AAA";
    let mut count = 0;
    for direction in instructions.chars().cycle() {
        let node = map.get(current).unwrap();

        match direction {
            'L' => current = &node.0,
            'R' => current = &node.1,
            _ => unreachable!()
        }
        count += 1;

        if current == "ZZZ" {
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
        assert_eq!(process(include_str!("./sample.txt")), 6);
    }
}
