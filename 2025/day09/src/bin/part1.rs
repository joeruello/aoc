use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 9).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (x, y) = l
                .split(',')
                .map(|d| d.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y)
        })
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 50);
    }
}
