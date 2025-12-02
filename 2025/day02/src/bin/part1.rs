use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    let ids = input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .flat_map(|l| {
            dbg!(l);
            let (a, b) = l.split_once('-').unwrap();

            [a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()]
        })
        .collect_vec();
    let mut sum = 0;
    for range in ids {
        for id in range {
            if !is_valid(id) {
                sum += id;
            }
        }
    }

    sum
}

fn is_valid(id: u64) -> bool {
    let id = format!("{id}");
    if id.len() % 2 == 1 {
        return true;
    }
    let idx = id.len() / 2;
    let start = &id[..idx];
    let end = &id[idx..];
    !start.eq(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 1227775554);
    }

    #[test]
    fn it_validates() {
        assert!(is_valid(1));
        assert!(!is_valid(11));
        assert!(!is_valid(123123));
        assert!(!is_valid(1188511885))
    }
}
