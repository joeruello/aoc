use std::collections::{HashMap, VecDeque};

fn main() {
    let input: String = common::AocInput::fetch(2023, 15).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u32 {
    let mut map: HashMap<u32, VecDeque<(String, u32)>> = HashMap::new();
    for input in input.split(',') {
        if input.contains('=') {
            let (label, focal_length) = input.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            map.entry(hash(label))
                .and_modify(|vec| {
                    if let Some(idx) = vec.iter().position(|(l, _)| l == label) {
                        vec[idx] = (label.to_owned(), focal_length)
                    } else {
                        vec.push_back((label.to_owned(), focal_length))
                    }
                })
                .or_insert(VecDeque::from([(label.to_string(), focal_length)]));
        } else {
            let label = &input[0..input.len() - 1];
            map.entry(hash(label)).and_modify(|v| {
                if let Some(idx) = v.iter().position(|(l, _)| l == label) {
                    v.remove(idx);
                }
            });
        }
    }

    let mut total = 0;
    for (key, values) in map {
        for (idx, (_, focal_len)) in values.into_iter().enumerate() {
            total += (key + 1) * (idx + 1) as u32 * focal_len;
        }
    }

    total
}

fn hash(input: &str) -> u32 {
    let mut current_value = 0;
    for c in input.chars() {
        assert!(c.is_ascii());
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 145);
    }
    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
