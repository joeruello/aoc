use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 22).unwrap().into();
    println!("Output: {}", process(&input, 2000));
}

fn process(input: &str, len_prices: usize) -> isize {
    let mut unique_sequences = HashSet::new();
    let sellers = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .map(|i| {
            let mut seqs = HashMap::new();
            let diffs = successors(Some(i), |n| Some(next_secret(*n)))
                .take(len_prices)
                .map(|n| n % 10)
                .tuple_windows()
                .map(|(a, b)| (b - a, b))
                .collect_vec();

            diffs
                .windows(4)
                .map(Vec::from)
                .map(|seq| {
                    let (_, price) = seq[seq.len() - 1];
                    let seq = seq.into_iter().map(|(a, _)| a).join(",");
                    (seq, price)
                })
                .for_each(|(seq, price)| {
                    unique_sequences.insert(seq.clone());
                    seqs.entry(seq).or_insert(price);
                });

            seqs
        })
        .collect_vec();

    let mut max_count = 0;

    for seq in unique_sequences {
        let mut count = 0isize;
        for seller in &sellers {
            if let Some(price) = seller.get(&seq) {
                count += price
            }
        }
        if count > max_count {
            max_count = count
        }
    }

    max_count
}

fn next_secret(a: isize) -> isize {
    let a = ((a * 64) ^ a) % 16777216;
    let a = ((a / 32) ^ a) % 16777216;
    ((a * 2048) ^ a) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_secrets() {
        assert_eq!(
            process(
                "1
2
3
2024",
                2000
            ),
            23
        )
    }
}
