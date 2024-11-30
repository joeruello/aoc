use std::collections::{BinaryHeap, HashMap, HashSet};
use toodee::{TooDee, TooDeeOps};

type Cordinate = (usize, usize);
type Grid = TooDee<Tile>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Direction)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    coords: Cordinate,
    cost: usize,
    dir: Direction,
}

// Reversed comparisons (other.cmp vs self.cmp) to make BinaryHeap a min heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours((x, y): Cordinate, grid: &Grid) -> Vec<(Cordinate, Direction)> {
    let n = (y > 0).then_some(((x, y.saturating_sub(1)), Direction::N));
    let s = (y < grid.num_rows() - 1).then_some(((x, y + 1), Direction::S));
    let w = (x > 0).then_some(((x.saturating_sub(1), y), Direction::W));
    let e = (x < grid.num_cols() - 1).then_some(((x + 1, y), Direction::E));

    vec![n, s, e, w].into_iter().filter_map(|f|
        if let Some((cords, dir)) = f {
            match grid[cords] {
                Tile::Path => Some((cords,dir)),
                Tile::Forest => None,
                Tile::Slope(slope_dir) => if slope_dir == dir {
                    Some((cords,dir))
                } else {
                    None
                }
            }
        } else {
            None
        }
    ).collect()
}

fn parse(input: &str) -> Grid {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    TooDee::from_vec(
        width,
        height,
        input
            .replace('\n', "")
            .trim()
            .chars()
            .map(|c| match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '>' => Tile::Slope(Direction::E),
                '<' => Tile::Slope(Direction::W),
                '^' => Tile::Slope(Direction::N),
                'v' => Tile::Slope(Direction::S),
                _ => panic!("unknown tile {c}")
            })
            .collect(),
    )
}



fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn find_start(grid: &Grid) -> Cordinate {
    let first_row = grid.rows().next().unwrap();
    let start_x = first_row.iter().position(|t| *t == Tile::Path).unwrap();

    (start_x, 0)
}

fn find_end(grid: &Grid) -> Cordinate {
    let row = grid.rows().last().unwrap();
    let start_x = row.iter().position(|t| *t == Tile::Path).unwrap();

    (start_x, grid.num_rows() -1)
}

fn process(input: &str) -> usize {
    let grid = parse(input);
    let start = find_start(&grid);  
    let end = find_end(&grid);  

    println!("{start:?} {end:?}");

    dijkstra(&grid, start, end).unwrap()
}

fn dijkstra(grid: &Grid, start: Cordinate, dest: Cordinate) -> Option<usize> {
    let mut distances = HashMap::<(Cordinate, Direction), usize>::new();
    let mut visited = HashSet::<Cordinate>::new();
    let mut heap = BinaryHeap::<State>::new();

    let mut max_x = start.0;
    let mut max_y = start.1;
    let mut iters = 0;
    heap.push(State {
        coords: start,
        cost: 0,
        dir: Direction::S
    });
    distances.insert((start, Direction::E), 0);

    let mut goal_distances = vec![];

    while let Some(State {
        coords,
        cost,
        dir,
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
            goal_distances.push(cost);
            continue;
        }

        if distances
            .get(&(coords, dir))
            .is_some_and(|&c| c > cost)
        {
            // We've already visited this node from this direction
            continue;
        }

        for (neighbour_coordinate, neighbour_dir) in neighbours(coords, grid) {
            let next = State {
                cost: cost +1,
                coords: neighbour_coordinate,
                dir: neighbour_dir
            };
            if dir.opposite() == neighbour_dir {
                continue;
            }
            if distances
                .get(&(neighbour_coordinate, neighbour_dir))
                .is_some_and(|&c| c >= next.cost)
            {
                // We've already found a cheaper way to get here
                continue;
            }

            distances.insert((next.coords, next.dir), next.cost);
            heap.push(next);
        }
        if iters % 10000 == 0 {
            println!("{iters} - Checking {coords:?} {dir:?} Max X: {max_x}, Max Y: {max_y}");

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

    println!("{goal_distances:?}");
    goal_distances.into_iter().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 405);
    }
}
