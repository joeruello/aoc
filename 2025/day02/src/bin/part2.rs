fn main() {
    let input: String = common::AocInput::fetch(2025, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .flat_map(|l| {
            let (a, b) = l.split_once('-').unwrap();

            [a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()]
        })
        .flatten()
        .filter(|id| !is_valid(*id))
        .sum()
}

fn is_valid(id: u64) -> bool {
    let s = format!("{id}");
    for i in 1..s.len() {
        if s == s[..i].repeat(s.len() / i) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 4174379265);
    }

    #[test]
    fn it_validates() {
        assert!(is_valid(1698522));
        assert!(!is_valid(11));
        assert!(!is_valid(123123));
        assert!(!is_valid(1188511885));
        assert!(!is_valid(824824824));
        assert!(is_valid(824824823));
        assert!(!is_valid(565656))
    }
}
