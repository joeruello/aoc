use itertools::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2023, 9).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i64 {
    let sequences: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    sequences
        .into_iter()
        .map(|seq| predict_next_value(&seq))
        .sum()
}

fn predict_next_value(sequence: &[i64]) -> i64 {
    let diffs: Vec<_> = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect();

    if diffs.iter().all(|n| *n == 0) {
        *sequence.last().unwrap()
    } else {
        sequence.last().unwrap() + predict_next_value(&diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 114);
    }
}
