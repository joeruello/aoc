use std::{
    cmp::Ordering,
    collections::BTreeMap,
};

fn main() {
    let input: String = common::AocInput::fetch(2023, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug, Eq)]
struct Hand {
    hand: Vec<u64>,
    freq: BTreeMap<u64, usize>,
    card_score: u64,
    bid: u64,
}

impl Hand {
    fn score(&self) -> u64 {
        let unique_count = self.freq.keys().count();
        let highest_freq = *self.freq.values().max().unwrap();
        let base = 10_000_000_000;

        let rank = match (unique_count, highest_freq) {
            (1, _) => 6 * base, // 5 a kind
            (2, 4) => 5 * base, // 4 of a kind
            (2, 3) => 4 * base, // full house
            (_, 3) => 3 * base, // three of a kind
            (3, 2) => 2 * base, // two pair
            (4, 2) => base,     // pair
            (_, 1) => 0,        // high card
            _ => unreachable!(),
        };

        println!("rank:{} {:?}", (rank / base), self.hand);
        rank + self.card_score
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

fn process(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|input| {
            let (hand, bid) = input.split_once(' ').unwrap();
            let hand: Vec<_> = hand
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    c => c.to_digit(10).unwrap() as u64,
                })
                .collect();

            let score: String = hand
                .clone()
                .into_iter()
                .map(|n| format!("{:0>2}", n))
                .collect();

            Hand {
                freq: hand
                    .clone()
                    .into_iter()
                    .fold(BTreeMap::new(), |mut map, n| {
                        map.entry(n).and_modify(|k| *k += 1).or_insert(1);
                        map
                    }),
                card_score: score.parse::<u64>().unwrap(),
                hand,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();

    dbg!(&hands);

    for hand in &hands {
        hand.score();
    }

    hands.sort();
    // hands.reverse();

    println!("\n\n\n =======");
    for hand in &hands {
        hand.score();
    }

    let scores: Vec<_> = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .collect();

    // dbg!(&scores);
    scores.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 6440);
    }
}
