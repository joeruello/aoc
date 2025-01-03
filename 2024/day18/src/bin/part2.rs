use std::collections::{BinaryHeap, HashMap};

use common::{Direction, DirectionOps};
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 18).unwrap().into();
    println!("Output: {:?}", process(&input, 71, 71));
}

#[derive(PartialEq, Eq)]
struct State((usize, usize), usize);
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1
            .cmp(&other.1)
            .reverse()
            .then_with(|| self.0.cmp(&other.0))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process(input: &str, width: usize, height: usize) -> (usize, usize) {
    let bytes: Vec<_> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            (x, y)
        })
        .collect();

    let grid: TooDee<u8> = TooDee::new(width, height);

    for i in 0..bytes.len() {
        if !find_exit(&grid, &bytes[0..=i]) {
            return bytes[i];
        }
    }
    panic!("answer not found");
}

fn find_exit(grid: &TooDee<u8>, bytes: &[(usize, usize)]) -> bool {
    let start = (0, 0);
    let goal = (grid.num_cols() - 1, grid.num_rows() - 1);
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::from([State(start, 0)]);
    while let Some(State(point, score)) = queue.pop() {
        if point == goal {
            return true;
        }

        if let Some(prev_score) = visited.get(&point) {
            if *prev_score <= score {
                continue;
            }
        }
        visited.insert(point, score);

        for dir in Direction::CARDINALS {
            if let Some(next) = grid.move_point(&point, dir) {
                if bytes.contains(&next) {
                    continue;
                }

                queue.push(State(next, score + 1));
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 7, 7), (6, 1));
    }
}
