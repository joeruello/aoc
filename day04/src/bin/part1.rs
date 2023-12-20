use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, game) = l.split_once(':').expect("Valid input");
            let (winnings_numbers, my_numbers) = game.split_once('|').unwrap();
            let winnings_numbers: HashSet<_> = winnings_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().expect("Valid input"))
                .collect();
            let my_numbers: HashSet<_> = my_numbers
                .split_whitespace()
                .map(|d| d.trim().parse::<u32>().expect("Valid input"))
                .collect();

            let matches: u32 = my_numbers.intersection(&winnings_numbers).count() as u32;

            if matches > 0 {
                2_u32.pow(matches- 1) // 1,2,4,6,8...
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
