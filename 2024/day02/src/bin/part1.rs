use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let levels = l
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect_vec();
            is_safe(&levels)
        })
        .count()
}

fn is_safe(levels: &[isize]) -> bool {
    let mut direction = None;
    for (a, b) in levels.iter().tuple_windows() {
        let diff = b - a;
        let signum = diff.signum();
        if direction.is_some_and(|dir| signum != dir) {
            return false;
        } else if direction.is_none() {
            direction = Some(signum)
        }

        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }
    true
}
