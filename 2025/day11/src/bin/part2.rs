use std::collections::{HashMap, HashSet, VecDeque};

use common::Itertools;

fn main() {
    let waypoints = vec![vec!["svr"], vec!["fft"], vec!["dac"], vec!["out"]];
    let input: String = common::AocInput::fetch(2025, 11).unwrap().into();
    println!("Output: {}", process(&input, waypoints));
}

fn process(input: &str, waypoints: Vec<Vec<&str>>) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut inv_graph: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|l| {
        let (src, dests) = l.split_once(": ").unwrap();
        let dests = dests
            .split_whitespace()
            .inspect(|d| {
                (inv_graph.entry(d).or_default()).push(src);
            })
            .collect_vec();
        graph.insert(src, dests);
    });

    let mut count = 1;

    // graph is a dag and there is an order (even though the prompt says there isnt)

    for (a, b) in waypoints.into_iter().tuple_windows() {
        let mut this_run = 0;
        for (src, dest) in a.into_iter().cartesian_product(b) {
            let sub = get_reachable(src, &graph);
            let sub_inv = get_reachable(dest, &inv_graph);
            let sub: HashSet<_> = sub.intersection(&sub_inv).copied().collect();

            this_run += count_paths(src, dest, &graph, &sub);
        }
        count *= this_run;
    }
    count
}

fn count_paths(
    from: &str,
    to: &str,
    graph: &HashMap<&str, Vec<&str>>,
    subgraph: &HashSet<&str>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(from);

    let mut solutions = 0;
    println!("Trying {from} -> {to}");

    while let Some(current) = queue.pop_front() {
        if let Some(next) = graph.get(current) {
            for next in next {
                if !subgraph.contains(next) {
                    continue;
                } else if next.eq(&to) {
                    solutions += 1;
                    continue;
                } else {
                    queue.push_front(next);
                }
            }
        }
    }

    println!("{from} -> {to} = {solutions}");
    solutions
}

fn get_reachable<'a>(from: &'a str, graph: &'a HashMap<&'a str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(from);
    queue.push_front(from);

    while let Some(current) = queue.pop_front() {
        if let Some(next) = graph.get(current) {
            for &next in next {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    visited
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        let waypoints = vec![vec!["svr"], vec!["fft"], vec!["dac"], vec!["out"]];
        assert_eq!(process(include_str!("./sample2.txt"), waypoints), 2);
    }
}
