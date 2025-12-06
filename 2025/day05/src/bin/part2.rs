fn main() {
    let input: String = common::AocInput::fetch(2025, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    let (fresh, _) = input.split_once("\n\n").unwrap();
    let mut fresh: Vec<_> = fresh
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();

            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect();

    fresh.sort_by_key(|(a, _)| *a);

    let ret = fresh[1..]
        .iter()
        .fold(vec![fresh[0].to_owned()], |mut acc, b| {
            let (a1, b1) = acc.pop().unwrap();
            let (a2, b2) = *b;

            if a2 <= b1 {
                acc.push((a1, b1.max(b2).to_owned()));
            } else {
                acc.push((a1, b1));
                acc.push(b.to_owned());
            }
            acc
        });

    ret.into_iter().map(|(a, b)| (b + 1) - a).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 14);
    }
}
