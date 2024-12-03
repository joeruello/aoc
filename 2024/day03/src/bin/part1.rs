use regex::Regex;

fn main() {
    let input: String = common::AocInput::fetch(2024, 3).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    Regex::new(r"(mul\(\d+,\d+\))")
        .unwrap()
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x])| {
            let (a, b) = x
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(",")
                .unwrap();

            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            a * b
        })
        .sum()
}
