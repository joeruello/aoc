use unescape::unescape;

fn main() {
    let input: String = common::AocInput::fetch(2015, 8).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let unquoted = l
                .strip_prefix('"')
                .and_then(|l| l.strip_suffix('"'))
                .unwrap();
            let u = unescape(unquoted).unwrap();
            println!("{l} -> {u}");
            let diff = l.len() - u.chars().count();
            l.len() + diff
        })
        .sum()
}
