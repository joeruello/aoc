use std::collections::{BinaryHeap, HashMap, HashSet};
use toodee::{TooDee, TooDeeOps};

type Cordinate = (usize, usize);
type Grid = TooDee<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    coords: Cordinate,
    cost: usize,
    dir: Direction,
    steps: usize,
}

// Reversed comparisons (other.cmp vs self.cmp) to make BinaryHeap a min heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.coords.cmp(&self.coords))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let grid = TooDee::from_vec(
        width,
        height,
        input
            .replace('\n', "")
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(),
    );

    dijkstra(&grid, (0, 0), (grid.num_cols() - 1, grid.num_rows() - 1)).unwrap()
}

fn dijkstra(grid: &Grid, start: Cordinate, dest: Cordinate) -> Option<usize> {
    let mut distances = HashMap::<(Cordinate, Direction, usize), usize>::new();
    let mut visited = HashSet::<Cordinate>::new();
    let mut heap = BinaryHeap::<State>::new();

    let mut max_x = start.0;
    let mut max_y = start.1;
    let mut iters = 0;
    heap.push(State {
        coords: start,
        cost: 0,
        dir: Direction::E,
        steps: 0,
    });
    distances.insert((start, Direction::E, 0), 0);

    while let Some(State {
        coords,
        cost,
        dir,
        steps,
    }) = heap.pop()
    {
        iters += 1;
        if coords.0 > max_x {
            max_x = coords.0;
        }

        if coords.1 > max_y {
            max_y = coords.1;
        }

        visited.insert(coords);
        if coords == dest {
            return Some(cost);
        }

        if distances
            .get(&(coords, dir, steps))
            .is_some_and(|&c| c < cost)
        {
            // We've already visited this node from this direction
            continue;
        }

        for (neighbour_coordinate, neighbour_dir) in neighbours(coords, grid) {
            let next = State {
                cost: cost + grid[neighbour_coordinate],
                coords: neighbour_coordinate,
                dir: neighbour_dir,
                steps: if neighbour_dir == dir { steps + 1 } else { 1 },
            };
            if next.steps > 3 || dir.opposite() == neighbour_dir {
                // Cant move more than 3 times in the same direction
                continue;
            }
            if distances
                .get(&(neighbour_coordinate, neighbour_dir, next.steps))
                .is_some_and(|&c| c <= next.cost)
            {
                // We've already found a cheaper way to get here
                continue;
            }

            distances.insert((next.coords, next.dir, next.steps), next.cost);
            heap.push(next);
        }
        if iters % 10000 == 0 {
            println!("{iters} - Checking {coords:?} {dir:?} {steps:?} Max X: {max_x}, Max Y: {max_y}");

            for y in 0..grid.num_rows() {
                for x in 0..grid.num_cols() {
                    if visited.contains(&(x,y)) {
                        print!("X")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }

        }
    }

    None
}

fn neighbours((x, y): Cordinate, grid: &Grid) -> Vec<(Cordinate, Direction)> {
    let n = (y > 0).then_some(((x, y.saturating_sub(1)), Direction::N));
    let s = (y < grid.num_rows() - 1).then_some(((x, y + 1), Direction::S));
    let w = (x > 0).then_some(((x.saturating_sub(1), y), Direction::W));
    let e = (x < grid.num_cols() - 1).then_some(((x + 1, y), Direction::E));

    vec![n, s, e, w].into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 102);
    }
}
