use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = common::AocInput::fetch(2023, 4).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u32 {
    let card_map = input
        .lines()
        .map(|l| {
            let (game_id, game) = l.split_once(':').expect("valid input");
            let game_id = game_id
                .replace("Card", "")
                .trim()
                .parse::<usize>()
                .expect("valid game id");
            let (winning_numbers, my_numbers) = game.split_once('|').unwrap();
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().expect("valid input"))
                .collect::<HashSet<_>>();
            let my_numbers = my_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().expect("valid input"))
                .collect::<HashSet<_>>();

            let matches = my_numbers.intersection(&winning_numbers).count();

            (game_id, matches)
        })
        .collect::<HashMap<_,_>>();

    let mut count = 0;
    let mut stack = card_map.keys().map(ToOwned::to_owned).collect::<Vec<_>>();
    let mut next_round: Vec<_> = vec![];
    while !stack.is_empty() {
        for card_id in stack {
            count += 1;
            let matches = *card_map.get(&card_id).expect("Should be valid card id");
            for x in (card_id + 1)..=card_id + matches {
                next_round.push(x);
            }
        }
        stack = next_round.clone();
        next_round = vec![];
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 30);
    }
}
