use std::{collections::HashMap, time::Instant};

use common::Itertools;

fn main() {
    let waypoints = vec!["svr", "fft", "dac", "out"];
    let input: String = common::AocInput::fetch(2025, 11).unwrap().into();
    let start = Instant::now();
    println!(
        "Output: {} ({:?})",
        process(&input, waypoints),
        start.elapsed()
    );
}

fn process(input: &str, waypoints: Vec<&str>) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|l| {
        let (src, dests) = l.split_once(": ").unwrap();
        let dests = dests.split_whitespace().collect_vec();
        graph.insert(src, dests);
    });

    let mut count = 1;

    // graph is a dag and there is an order (even though the prompt says there isnt)

    for (src, dest) in waypoints.into_iter().tuple_windows() {
        let mut memo = HashMap::with_capacity(graph.len());
        count *= count_paths(src, dest, &graph, &mut memo);
    }
    count
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
        let waypoints = vec!["svr", "fft", "dac", "out"];
        assert_eq!(process(include_str!("./sample2.txt"), waypoints), 2);
    }
}
