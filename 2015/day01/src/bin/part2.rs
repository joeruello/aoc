fn main() {
    let input: String = common::AocInput::fetch(2015, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut count = 0;

    for (i, n) in input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .enumerate()
    {
        count += n;
        if count == -1 {
            return i + 1;
        }
    }
    unreachable!()
}
