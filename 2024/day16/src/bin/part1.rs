use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use common::{Direction, DirectionOps, Itertools};
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 16).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State((usize, usize), Direction, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let start = grid.find('S').expect("should have a start");

    let mut heap = BinaryHeap::from([State(start, Direction::E, 0)]);
    let mut lowest_score = usize::MAX;
    let mut visited = HashMap::new();

    while let Some(State(point, dir, score)) = heap.pop() {
        if score > lowest_score {
            continue;
        }

        if grid[point] == 'E' {
            if score.cmp(&lowest_score) == Ordering::Less {
                lowest_score = score;
            }
            continue;
        }
        if let Some(last_score) = visited.get(&(point, dir)) {
            if *last_score < score {
                continue;
            }
        }

        for n_dir in Direction::CARDINALS {
            if let Some(neighbour) = grid.move_point(&point, n_dir.xy()) {
                if grid[neighbour] == '#' {
                    continue;
                }
                let score_inc = if n_dir == dir { 1 } else { 1001 };
                let new_score = score + score_inc;
                if new_score > lowest_score {
                    continue;
                }
                let new = State(neighbour, n_dir, score + score_inc);
                heap.push(new);
            }
        }

        visited.insert((point, dir), score);
    }
    lowest_score
}
