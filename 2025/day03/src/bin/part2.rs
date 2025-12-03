fn main() {
    let input: String = common::AocInput::fetch(2025, 3).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    let batteries: Vec<Vec<u64>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap().into()).collect())
        .collect();

    let mut sum = 0;
    for bat in batteries {
        let mut s: u64 = 0;
        let mut i = 0;
        for place in (0..12).rev() {
            let (j, num) = &bat[i..bat.len() - place]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, b)| *b)
                .unwrap()
                .to_owned();

            i += *j + 1;
            s += **num * 10u64.pow(place as u32)
        }
        sum += s;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process("987654321111111"), 987654321111);
        assert_eq!(process(include_str!("./sample.txt")), 3121910778619);
    }
}
