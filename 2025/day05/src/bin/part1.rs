use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let fresh = fresh
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();

            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect_vec();

    available
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| fresh.iter().any(|(a, b)| id > a && id <= b))
        .count()
}
