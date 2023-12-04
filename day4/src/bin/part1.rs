use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, game) = l.split_once(':').expect("valid input");
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

            if matches > 0 {
                2u32.pow(matches as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 13);
    }
}
