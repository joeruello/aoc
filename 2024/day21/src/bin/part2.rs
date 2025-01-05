use std::{
    collections::HashMap,
    iter::{once, repeat_n},
};

use common::{DirectionOps, Itertools, TooDee};

type Cache = HashMap<(usize, char, char), usize>;

fn main() {
    let input: String = common::AocInput::fetch(2024, 21).unwrap().into();
    println!("Output: {}", process(&input, 25));
}

fn process(input: &str, levels: usize) -> usize {
    let codes = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let numeric_pad = TooDee::from_vec(
        3,
        4,
        vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', '_', '0', 'A'],
    );
    let directional_pad = TooDee::from_vec(3, 2, vec!['_', '^', 'A', '<', 'v', '>']);

    let mut cache = HashMap::new();
    for l in 1..=levels {
        preload_cache(&mut cache, &directional_pad, l);
    }
    preload_cache(&mut cache, &numeric_pad, levels + 1);

    let mut complexity = 0;
    for code in codes {
        let num = code[0..3].iter().join("").parse::<usize>().unwrap();

        let moves = seq_cost(&mut cache, levels + 1, &code);

        complexity += moves * num;
        println!(
            "{:?}: complexity: {} x {} = {}",
            code.into_iter().join(""),
            moves,
            num,
            moves * num
        );
    }

    complexity
}

fn cost(cache: &Cache, level: usize, a: char, b: char) -> usize {
    if level == 0 {
        1
    } else {
        *cache
            .get(&(level, a, b))
            .unwrap_or_else(|| panic!("missing key ({},{},{})", level, a, b))
    }
}

fn seq_cost(cache: &mut Cache, level: usize, seq: &[char]) -> usize {
    once(&'A')
        .chain(seq.iter())
        .tuple_windows()
        .map(|(a, b)| cost(cache, level, *a, *b))
        .sum()
}

fn preload_cache(cache: &mut Cache, pad: &TooDee<char>, level: usize) {
    let data = pad.data();
    for (a, b) in data.iter().copied().cartesian_product(data.iter().copied()) {
        let seq = shortest_path(pad, a, b);
        let cost = seq_cost(cache, level - 1, &seq);
        cache.insert((level, a, b), cost);
    }
}

fn shortest_path(pad: &TooDee<char>, a: char, b: char) -> Vec<char> {
    let mut seq = vec![];
    let pos = pad.find(a).unwrap();
    let target = pad.find(b).unwrap();
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

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 2), 126384);
    }
}
