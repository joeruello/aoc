fn main() {
    let input: String = common::AocInput::fetch(2023, 1).unwrap().into();
    let lines = input.lines();
    let output: u32 = lines
        .map(|f| {
            let numbers: Vec<_> = f.chars().filter(|c| c.is_numeric()).collect();
            let first = numbers.first().expect("should be a first number");
            let last = numbers.last().expect("Should be a second number");
            format!("{first}{last}")
                .parse::<u32>()
                .expect("should be a valid nimber")
        })
        .sum();

    dbg!(output);
}
