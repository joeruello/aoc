use std::collections::{BinaryHeap, HashMap, HashSet};

use common::{Direction, DirectionOps, Itertools};
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 16).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State((usize, usize), Direction, usize, Vec<(usize, usize)>);

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

    let mut heap = BinaryHeap::from([State(start, Direction::E, 0, vec![])]);
    let mut lowest_score = usize::MAX;
    let cardinals = [Direction::N, Direction::E, Direction::S, Direction::W];
    let mut visited = HashMap::new();

    let mut final_paths = vec![];

    while let Some(State(point, dir, score, path)) = heap.pop() {
        if score > lowest_score {
            continue;
        }

        if grid[point] == 'E' {
            match score.cmp(&lowest_score) {
                std::cmp::Ordering::Less => {
                    lowest_score = score;
                    final_paths = vec![path];
                }
                std::cmp::Ordering::Equal => final_paths.push(path),
                std::cmp::Ordering::Greater => {}
            }
            continue;
        }
        if let Some(last_score) = visited.get(&(point, dir)) {
            if *last_score < score {
                continue;
            }
        }
        let mut new_path = path.clone();
        new_path.push(point);

        for n_dir in cardinals {
            if let Some(neighbour) = grid.move_point(&point, n_dir.xy()) {
                if grid[neighbour] == '#' {
                    continue;
                }
                let score_inc = if n_dir == dir { 1 } else { 1001 };
                let new_score = score + score_inc;
                if new_score > lowest_score {
                    continue;
                }
                let new = State(neighbour, n_dir, score + score_inc, new_path.clone());
                heap.push(new);
            }
        }

        visited.insert((point, dir), score);
    }

    let tiles: HashSet<_> = final_paths.into_iter().flatten().collect();

    tiles.len() + 1 // add one for the ending tile
}
