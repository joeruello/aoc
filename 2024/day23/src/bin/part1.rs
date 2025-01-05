use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = common::AocInput::fetch(2024, 23).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").expect("- seperated");
        graph
            .entry(a)
            .and_modify(|e| {
                e.insert(b);
            })
            .or_insert(HashSet::from([b]));
        graph
            .entry(b)
            .and_modify(|e| {
                e.insert(a);
            })
            .or_insert(HashSet::from([a]));
    }

    let mut triples = HashSet::new();
    let empty_hashset = HashSet::new();
    for node in graph.keys() {
        if !node.starts_with("t") {
            continue;
        }
        let neighbours = graph.get(node).unwrap_or(&empty_hashset);

        for neighbour in neighbours {
            for n2 in graph.get(neighbour).unwrap_or(&empty_hashset) {
                if neighbours.contains(n2) {
                    triples.insert(sort((node, neighbour, n2)));
                }
            }
        }
    }
    println!("{:?}", triples);
    triples.len()
}

pub fn sort<T: Eq + Ord + Copy>(tpl: (T, T, T)) -> (T, T, T) {
    let (a, b, c) = tpl;
    let mut v = [a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 7);
    }
}
