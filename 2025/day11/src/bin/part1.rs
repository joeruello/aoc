use std::{collections::HashMap, time::Instant};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 11).unwrap().into();

    let start = Instant::now();
    println!("Output: {} ({:?})", process(&input), start.elapsed());
}

fn process(input: &str) -> usize {
    let mut graph = HashMap::new();

    input.lines().for_each(|l| {
        let (src, dests) = l.split_once(": ").unwrap();
        let dests = dests.split_whitespace().collect_vec();
        graph.insert(src, dests);
    });

    let mut memo = HashMap::new();
    count_paths("you", "out", &graph, &mut memo)
}

fn count_paths<'a>(
    from: &'a str,
    to: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&c) = memo.get(from) {
        return c;
    }

    if from == to {
        return 1;
    }

    let mut sum = 0;
    for n in graph.get(from).into_iter().flatten() {
        sum += count_paths(n, to, graph, memo);
    }

    memo.insert(from, sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 5);
    }
}
