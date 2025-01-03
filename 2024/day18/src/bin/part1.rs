use std::collections::{BinaryHeap, HashMap, HashSet};

use common::{Direction, DirectionOps};
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 18).unwrap().into();
    println!("Output: {}", process(&input, 71, 71, 1024));
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

fn process(input: &str, width: usize, height: usize, count: usize) -> usize {
    let bytes: HashSet<_> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            (x, y)
        })
        .take(count)
        .collect();

    let grid: TooDee<u8> = TooDee::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if bytes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    let start = (0, 0);
    let goal = (width - 1, height - 1);
    let mut visited = HashMap::new();
    let mut lowest_score = usize::MAX;

    let mut queue = BinaryHeap::from([State(start, 0)]);

    let cardinals = [Direction::N, Direction::E, Direction::S, Direction::W];
    while let Some(State(point, score)) = queue.pop() {
        if score >= lowest_score {
            continue;
        }
        if point == goal {
            if score < lowest_score {
                lowest_score = score;
            }
            continue;
        }

        if let Some(prev_score) = visited.get(&point) {
            if *prev_score < score {
                continue;
            }
        }
        visited.insert(point, score);
        for dir in cardinals {
            if let Some(next) = grid.move_point(&point, dir.xy()) {
                if bytes.contains(&next) {
                    continue;
                }
                if (score + 1) > lowest_score {
                    continue;
                }

                queue.push(State(next, score + 1));
            }
        }

        println!("{} -> {:?}", queue.len(), (point, score))
    }

    lowest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 7, 7, 12), 22);
    }
}
