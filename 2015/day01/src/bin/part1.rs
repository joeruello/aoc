fn main() {
    let input: String = common::AocInput::fetch(2015, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> isize {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}
