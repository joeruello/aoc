use std::collections::{HashMap, HashSet, VecDeque};

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 11).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut graph = HashMap::new();

    input.lines().for_each(|l| {
        let (src, dests) = l.split_once(": ").unwrap();
        let dests = dests.split_whitespace().collect_vec();
        graph.insert(src, dests);
    });

    let mut queue = VecDeque::new();
    let mut initial = HashSet::new();
    initial.insert("you");
    queue.push_front(("you", initial));

    let mut solutions = 0;

    while let Some((current, path)) = queue.pop_front() {
        if current == "out" {
            solutions += 1;
            continue;
        }

        if let Some(next) = graph.get(current) {
            for next in next {
                if path.contains(next) {
                    continue;
                }
                let mut path = path.clone();
                path.insert(current);
                queue.push_back((next, path));
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 5);
    }
}
