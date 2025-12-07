use std::collections::HashMap;

fn main() {
    let input: String = common::AocInput::fetch(2025, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

type Col = (Vec<u64>, char);

fn process(input: &str) -> u64 {
    let mut problems: HashMap<usize, Col> = HashMap::new();

    input.lines().for_each(|l| {
        for (idx, col) in l.split_whitespace().enumerate() {
            let entry = problems.entry(idx).or_insert((vec![], '+'));

            match col.parse() {
                Ok(n) => entry.0.push(n),
                Err(_) => match col {
                    "+" => entry.1 = '+',
                    "*" => entry.1 = '*',
                    _ => panic!("ohno"),
                },
            }
        }
    });

    problems
        .into_values()
        .map(|(nums, op)| match op {
            '+' => nums.into_iter().sum::<u64>(),
            '*' => nums.into_iter().product(),
            _ => panic!("ohno"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 4277556);
    }
}
