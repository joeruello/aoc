use std::fmt::Write;
use std::{cmp::Ordering, collections::BTreeMap};
fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

#[derive(Debug, Eq, Clone)]
struct Hand {
    pub hand: Vec<u64>,
    bid: u64,
    rank_score: u64,
    card_score: u64,
}   

impl Hand {
    fn new(hand: Vec<u64>, bid: u64) -> Self {
        let score: String = hand
            .clone()
            .into_iter()
            .fold(String::new(), |mut s, n| {
                let _ = write!(s, "{:0>2}", n);
                s
            });

        let freq = hand
            .clone()
            .into_iter()
            .fold(BTreeMap::new(), |mut map, n| {
                map.entry(n).and_modify(|k| *k += 1).or_insert(1);
                map
            });
        let card_score = score.parse::<u64>().unwrap();

        Hand {
            bid,
            rank_score: Self::calc_score(hand.clone(), freq, bid),
            card_score,
            hand,
        }
    }

    fn score(&self) -> u64 {
        self.rank_score + self.card_score
    }

    fn calc_score(hand: Vec<u64>, freq: BTreeMap<u64, usize>, bid: u64) -> u64 {
        let unique_count = freq.keys().count();
        let highest_freq = freq.values().max().unwrap();
        let num_jokers = *freq.get(&1).unwrap_or(&0);
        let base = 10_000_000_000;

        if num_jokers > 0 && num_jokers < 5 {
            let mut highest_score: Option<Hand> = None;
            for wildcard in (2..=14).rev() {
                if !(freq.contains_key(&wildcard)) {
                    continue;
                }
                let mut new_hand = hand.clone();
                for n in new_hand.iter_mut() {
                    if *n == 1 {
                        *n = wildcard;
                        break;
                    }
                }
                let new_hand = Hand::new(new_hand, bid);
                if highest_score.is_none()
                    || highest_score
                        .as_ref()
                        .is_some_and(|s| s.rank_score < new_hand.rank_score)
                {
                    highest_score = Some(new_hand)
                }
            }

            let highest = highest_score.unwrap();
            highest.rank_score
        } else {
            match (unique_count, highest_freq) {
                (1, _) => 6 * base, // 5 a kind
                (2, 4) => 5 * base, // 4 of a kind
                (2, 3) => 4 * base, // full house
                (_, 3) => 3 * base, // three of a kind
                (3, 2) => 2 * base, // two pair
                (4, 2) => base,     // pair
                (_, 1) => 0,        // high card
                _ => unreachable!(),
            }
        }
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
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    c => c.to_digit(10).unwrap() as u64,
                })
                .collect();
            println!("Hand: {:?}", hand);
            Hand::new(hand, bid.parse().unwrap())
        })
        .inspect(|hand| println!("Final: {:?}\n", hand.hand))
        .collect();

    hands.sort();

    let scores: Vec<_> = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .collect();

    scores.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 5905);
    }
}
