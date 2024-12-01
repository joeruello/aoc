fn main() {
    let input: String = common::AocInput::fetch(2024, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("   ").unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            (a, b)
        })
        .collect();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}
