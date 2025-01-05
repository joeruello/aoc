use std::iter::repeat_n;

use common::{DirectionOps, Itertools, TooDee};

fn main() {
    let input: String = common::AocInput::fetch(2024, 21).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let codes = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let numeric_pad = TooDee::from_vec(
        3,
        4,
        vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', '_', '0', 'A'],
    );
    let directional_pad = TooDee::from_vec(3, 2, vec!['_', '^', 'A', '<', 'v', '>']);

    let mut complexity = 0;

    for code in codes {
        let num = code[0..3].iter().join("").parse::<usize>().unwrap();

        let mut seq = shortest_path(&numeric_pad, &code);
        for _ in 0..2 {
            seq = shortest_path(&directional_pad, &seq);
        }

        let moves = seq.len();

        complexity += moves * num;
        println!(
            "{}: {} complexity: {} x {} = {}",
            code.into_iter().join(""),
            seq.into_iter().join(""),
            moves,
            num,
            moves * num
        );
    }

    complexity
}

fn shortest_path(pad: &TooDee<char>, code: &[char]) -> Vec<char> {
    let mut seq = vec![];
    let mut pos = pad.find('A').unwrap();
    for char in code {
        let target = pad.find(*char).unwrap();
        let gap = pad.find('_').unwrap();

        let dx = target.0.abs_diff(pos.0);
        let dy = target.1.abs_diff(pos.1);
        let sx = repeat_n(if target.0 > pos.0 { '>' } else { '<' }, dx);
        let sy = repeat_n(if target.1 > pos.1 { 'v' } else { '^' }, dy);

        if target.0 > pos.0 && (pos.0, target.1) != gap {
            seq.extend(sy);
            seq.extend(sx);
        } else if (target.0, pos.1) != gap {
            seq.extend(sx);
            seq.extend(sy);
        } else {
            seq.extend(sy);
            seq.extend(sx);
        }

        seq.push('A');
        pos = target;
    }

    println!("{}", seq.clone().into_iter().join(""));
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 126384);
    }
}
