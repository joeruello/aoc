fn main() {
    let input: String = common::AocInput::fetch(2025, 3).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u32 {
    let batteries: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut sum = 0;
    for bat in batteries {
        let (i, tens) = &bat[..bat.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, b)| *b)
            .unwrap();
        dbg!(&bat, &tens);
        let ones = dbg!(&bat[*i + 1..bat.len()]).iter().max().unwrap();

        sum += dbg!((**tens * 10) + *ones)
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 358);
    }
}
