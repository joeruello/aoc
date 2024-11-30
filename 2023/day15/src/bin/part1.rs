fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> u32 {
    let mut current_value = 0;
    for c in input.chars() {
        assert!(c.is_ascii());
        current_value +=c as u32;
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
        assert_eq!(process(include_str!("./sample.txt")), 1320);
    }
}
