use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    let mappings: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|f| {
            let mut first = Option::None;
            let mut last = Option::None;
            for (digit, value) in &mappings {
                for (idx, _) in f.match_indices(digit) {
                    match first {
                        Some((first_idx, _)) => {
                            if idx < first_idx {
                                first = Some((idx, *value));
                            }
                        }
                        None => first = Some((idx, *value)),
                    };
                    match last {
                        Some((last_idx, _)) => {
                            if idx > last_idx {
                                last = Some((idx, *value));
                            }
                        }
                        None => last = Some((idx, *value)),
                    };
                }
            }

            let (_, first) = first.expect("Should be a valid number");
            let (_, last) = last.expect("Should be a valid number");
            format!("{first}{last}")
                .parse::<u32>()
                .expect("should be a valid nimber")
        })
        .sum()
}
