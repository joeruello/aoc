use itertools::Itertools;
use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<String, HashSet<String>>;
type EdgeKey = (String, String);

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let graph = parse(input);
    loop {
        let cuts = find_cuts(&graph);
        let cut_graph = do_cuts(&graph, &cuts);

        let (a, b) = cuts.first().expect("Should always be exactly 3 cuts");
        let size_a = count_reachable_nodes(a, &cut_graph);
        let size_b = count_reachable_nodes(b, &cut_graph);

        println!("Original Graph size: {}", graph.len());
        println!("Cuts to make: {cuts:?}");
        println!("Graph A Size: {size_a}");
        println!("Graph B Size: {size_b}");

        if size_a + size_b == graph.len() {
            return size_a * size_b;
        } else {
            println!("Failed to find cut, trying again :)")
        }
    }
}

fn parse(input: &str) -> Graph {
    let mut edges: Graph = HashMap::new();
    for line in input.lines() {
        let (src, dests) = line.split_once(": ").unwrap();
        for dest in dests.split_whitespace() {
            edges
                .entry(dest.to_string())
                .and_modify(|e| {
                    e.insert(src.to_string());
                })
                .or_insert_with(|| HashSet::from([src.to_string()]));

            edges
                .entry(src.to_string())
                .and_modify(|e| {
                    e.insert(dest.to_string());
                })
                .or_insert_with(|| HashSet::from([dest.to_string()]));
        }
    }
    edges
}

fn find_cuts(graph: &Graph) -> Vec<EdgeKey> {
    let rng = &mut rand::thread_rng();
    let mut frequencies: HashMap<EdgeKey, usize> = HashMap::new();

    // Randomly choose 2 nodes and find a path between then, recording the frequencies of
    // visiting each edge. Assumption is that over a large enough sample that "bridge" nodes
    // we're looking to cut will have the highest frequencies
    for _ in 0..500 {
        let (a, b) = graph
            .keys()
            .choose_multiple(rng, 2)
            .into_iter()
            .collect_tuple()
            .unwrap();
        breadth_first_search(a, b, graph, &mut frequencies)
    }

    // Sort candidate cuts by frequency
    let candidates = frequencies.iter_mut().sorted_by_key(|(_node, count)| **count).rev();

    // Find the top 3 cuts, we assume we that a node can't be
    // directly involved in more than one cut
    let mut visited = HashSet::new();
    let mut cuts = vec![];

    for ((a, b), _) in candidates {
        if visited.contains(a) || visited.contains(b) {
            continue;
        }
        cuts.push((a.to_string(), b.to_string()));
        visited.insert(a);
        visited.insert(b);
        if cuts.len() == 3 {
            break;
        }
    }
    cuts
}

fn do_cuts(graph: &Graph, cuts: &[EdgeKey]) -> Graph {
    let mut graph = graph.clone();
    for (a, b) in cuts.iter() {
        graph.entry(a.to_string()).and_modify(|edges| {
            edges.remove(b);
        });

        graph.entry(b.to_string()).and_modify(|edges| {
            edges.remove(a);
        });
    }
    graph
}

fn breadth_first_search(
    a: &str,
    b: &str,
    edges: &Graph,
    frequencies: &mut HashMap<EdgeKey, usize>,
) {
    let mut queue = VecDeque::from([a]);
    let mut visited = HashSet::new();
    while let Some(src) = queue.pop_front() {
        visited.insert(src);
        if src == b {
            return;
        }
        for dest in edges.get(src).unwrap() {
            if !visited.contains(dest.as_str()) {
                frequencies
                    .entry(create_key(src, dest))
                    .and_modify(|f| *f += 1)
                    .or_insert(1);

                queue.push_back(dest)
            }
        }
    }
}

fn create_key(a: &str, b: &str) -> EdgeKey {
    if a > b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

fn count_reachable_nodes(node: &str, edges: &Graph) -> usize {
    let mut queue = VecDeque::from([node]);
    let mut visited = HashSet::new();
    while let Some(node) = queue.pop_front() {
        visited.insert(node);
        for edge in edges.get(node).unwrap() {
            if !visited.contains(edge.as_str()) {
                queue.push_back(edge)
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 54);
    }
}
