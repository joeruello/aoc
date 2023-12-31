use itertools::*;
use rand::seq::IteratorRandom;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};
fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut nodes = HashSet::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let rng = &mut rand::thread_rng();

    for line in input.lines() {
        let (src, dests) = line.split_once(": ").unwrap();
        let dests: Vec<_> = dests.split_whitespace().collect();

        nodes.insert(src.to_string());
        nodes.extend(dests.iter().map(|s| s.to_string()));

        for dest in dests {
            nodes.insert(dest.to_string());
            edges
                .entry(dest.to_string())
                .and_modify(|e| e.push(src.to_string()))
                .or_insert_with(|| vec![src.to_string()]);

            edges
                .entry(src.to_string())
                .and_modify(|e| e.push(dest.to_string()))
                .or_insert_with(|| vec![dest.to_string()]);
        }
    }

    loop {
        let mut frequencies: HashMap<(String, String), usize> = HashMap::new();
        for _ in 0..1000 {
            let (a, b) = nodes
                .iter()
                .choose_multiple(rng, 2)
                .into_iter()
                .collect_tuple()
                .unwrap();
            search(a, b, &edges, &mut frequencies)
        }

        let candidates = frequencies
            .iter_mut()
            .sorted_by_key(|(_, v)| **v)
            .rev()
            .collect_vec();

        let mut visited = HashSet::new();
        let mut cuts = vec![];

        // Find the top 3 cuts, we assume we only cut one edge once
        for ((a, b), _) in candidates.into_iter() {
            if visited.contains(a) || visited.contains(b) {
                continue;
            }
            cuts.push((a, b));
            visited.insert(a);
            visited.insert(b);
            if cuts.len() == 3 {
                break;
            }
        }

        let mut cut_edges = edges.clone();

        for (a, b) in cuts.iter() {
            cut_edges.entry(a.to_string()).and_modify(|edges| {
                let idx = edges
                    .iter()
                    .position(|e| e == *b)
                    .expect("Should be an edge");
                edges.remove(idx);
            });

            cut_edges.entry(b.to_string()).and_modify(|edges| {
                let idx = edges
                    .iter()
                    .position(|e| e == *a)
                    .expect("Should be an edge");
                edges.remove(idx);
            });
        }

        let (a, b) = cuts.first().unwrap();
        let size_a = count_graph(a, &cut_edges);
        let size_b = count_graph(b, &cut_edges);

        println!("Original Graph size: {}", nodes.len());
        println!("Cuts to made: {cuts:?}");
        println!("Graph A Size: {size_a}");
        println!("Graph B Size: {size_b}");

        if size_a + size_b == nodes.len() {
            return size_a * size_b;
        } else {
            println!("Failed to find cut, trying again :)")
        }
    }
}

fn search(
    a: &str,
    b: &str,
    edges: &HashMap<String, Vec<String>>,
    frequencies: &mut HashMap<(String, String), usize>,
) {
    let mut queue = VecDeque::from([a]);
    let mut visisted = HashSet::new();
    while let Some(node) = queue.pop_front() {
        visisted.insert(node);
        if node == b {
            return;
        }
        for edge in edges.get(node).unwrap() {
            if !visisted.contains(edge.as_str()) {
                frequencies
                    .entry(create_key(node, edge))
                    .and_modify(|f| *f += 1)
                    .or_insert(1);

                queue.push_back(edge)
            }
        }
    }
}

fn count_graph(a: &str, edges: &HashMap<String, Vec<String>>) -> usize {
    let mut queue = VecDeque::from([a]);
    let mut visisted = HashSet::new();
    while let Some(node) = queue.pop_front() {
        visisted.insert(node);
        for edge in edges.get(node).unwrap() {
            if !visisted.contains(edge.as_str()) {
                queue.push_back(edge)
            }
        }
    }
    visisted.len()
}

fn create_key(a: &str, b: &str) -> (String, String) {
    if a > b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 54);
    }
}
