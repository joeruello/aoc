fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut input = input.lines();
    let time = dbg!(input
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(' ', "")
        .parse::<usize>()
        .unwrap());
    let distance = input
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    (1..time)
        .filter(|speed| speed * (time - speed) > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 71503);
    }
}
