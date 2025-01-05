use std::iter::successors;

fn main() {
    let input: String = common::AocInput::fetch(2024, 22).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|i| nth_secret_number(i, 2000))
        .sum()
}

fn next_secret(a: usize) -> usize {
    let a = ((a * 64) ^ a) % 16777216;
    let a = ((a / 32) ^ a) % 16777216;
    ((a * 2048) ^ a) % 16777216
}

fn nth_secret_number(initial: usize, iterations: usize) -> usize {
    successors(Some(initial), |n| Some(next_secret(*n)))
        .nth(iterations)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_secrets() {
        assert_eq!(nth_secret_number(123, 10), 5908254)
    }
}
