fn main() {
    let input: String = common::AocInput::fetch(2015, 4).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    println!("{input}");
    let trimmed = input.trim();
    for n in 0.. {
        let hash = md5::compute(format!("{trimmed}{n}"));
        let hex = format!("{hash:x}");
        if &hex[0..6] == "000000" {
            println!("{hex}");
            return n;
        }
    }
    unreachable!()
}
