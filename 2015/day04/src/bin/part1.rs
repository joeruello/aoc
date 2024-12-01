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
        if &hex[0..5] == "00000" {
            println!("{hex}");
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("abcdef");
        assert_eq!(result, 609043);
        let result = process("pqrstuv");
        assert_eq!(result, 1048970);
    }
}
