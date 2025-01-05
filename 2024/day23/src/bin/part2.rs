use std::collections::{HashMap, HashSet};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 23).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> String {
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

    let r: HashSet<&str> = HashSet::new();
    let mut p: HashSet<&str> = graph.keys().copied().collect();
    let mut x: HashSet<&str> = HashSet::new();

    let mut cliques: Vec<Vec<&str>> = Vec::new();
    bron_kerbosch_v2(&r, &mut p, &mut x, &graph, &mut cliques);

    cliques
        .into_iter()
        .max_set_by_key(|v| v.len())
        .first()
        .unwrap()
        .join(",")
}

// stolen from https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch_v2<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    g: &HashMap<&str, HashSet<&'a str>>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<&str> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<&str> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v);

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p.intersection(&neighbors_v).cloned().collect();

            // New X is X ∩ N(v)
            let mut new_x = x.intersection(&neighbors_v).cloned().collect();

            // Recursive call
            bron_kerbosch_v2(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), "co,de,ka,ta");
    }
}
