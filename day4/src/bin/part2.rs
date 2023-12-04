use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    let card_map: HashMap<usize, usize> = input
        .lines()
        .map(|l| {
            let (game_id, game) = l.split_once(':').expect("valid input");
            let game_id = game_id
                .replace("Card", "")
                .trim()
                .parse::<usize>()
                .expect("valid game id");
            let (winnings_numbers, my_numbers) = game.split_once('|').unwrap();
            let winnings_numbers: HashSet<_> = winnings_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().unwrap())
                .collect();
            let my_numbers: HashSet<_> = my_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().unwrap())
                .collect();

            let matches: usize = my_numbers.intersection(&winnings_numbers).count();

            (game_id, matches)
        })
        .collect();

    let mut count = 0;
    let mut stack: Vec<_> = card_map.keys().map(|x| x.to_owned()).collect();
    let mut histogram: HashMap<usize, usize> = HashMap::new();
    let mut next_round: Vec<_> = vec![];
    while !stack.is_empty() {
        for card_id in stack.iter() {
            let matches = card_map.get(card_id).expect("Should be valid card id");
            histogram.entry(*card_id).and_modify(|e| *e += 1).or_insert(0);
            count += 1;
            for x in (*card_id + 1)..=*card_id + *matches {
                next_round.push(x);
            }
        }
        stack = next_round.clone();
        next_round = vec![];
    }
    dbg!(histogram);
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
