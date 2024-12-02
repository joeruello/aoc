use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .filter(|l| -> bool {
            let levels = l
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            match is_safe(&levels) {
                true => true,
                false => levels
                    .clone()
                    .into_iter()
                    .combinations(levels.len() - 1)
                    .any(|l| is_safe(&l)),
            }
        })
        .count()
}

fn is_safe(levels: &Vec<usize>) -> bool {
    let reversed = levels.clone().into_iter().rev().collect_vec();
    let mut sorted = levels.clone();
    sorted.sort();

    if *levels != sorted && reversed != sorted {
        return false;
    }

    for (a, b) in sorted.into_iter().tuple_windows() {
        let diff = b - a;
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process("1 3 2 4 5"), 1);
    }
}
