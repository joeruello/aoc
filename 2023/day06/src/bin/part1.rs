fn main() {
    let input: String = common::AocInput::fetch(2023, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut input = input.lines();
    let times: Vec<_> = input
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<_> = input
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            (1..time)
                .filter(|speed| speed * (time - speed) > distance)
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 288);
    }
}
