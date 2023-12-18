use std::{collections::HashSet, vec};

use toodee::{TooDee, TooDeeOps};

type Cordinate = (usize, usize);
type Grid = TooDee<(char, HashSet<Direction>)>;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

fn main() {
    let input = include_str!("./input.txt");
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
            .map(|c| (c, HashSet::<Direction>::new()))
            .collect(),
    );

    let top = (0..grid.num_cols()).map(|i| ((i, 0), Direction::S));
    let bottom = (0..grid.num_cols()).map(|i| ((i, grid.num_rows() - 1), Direction::N));
    let left = (0..grid.num_rows()).map(|i| ((0, i), Direction::E));
    let right = (0..grid.num_rows()).map(|i| ((grid.num_cols() - 1, i), Direction::W));

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|(cordindate, direction)| {
            let mut cloned_grid = grid.clone();
            walk(cordindate, direction, &mut cloned_grid);
            count_energised(&cloned_grid)
        })
        .max()
        .unwrap()
}

fn count_energised(grid: &Grid) -> usize {
    grid.into_iter().filter(|(_, set)| !set.is_empty()).count()
}

fn walk(cordinate: Cordinate, direction: Direction, grid: &mut Grid) {
    let (tile, directions) = &mut grid[cordinate];
    if directions.contains(&direction) {
        return;
    }
    directions.insert(direction);
    let next_directions = match tile {
        '.' => vec![direction],
        '|' => match direction {
            Direction::N | Direction::S => vec![direction],
            Direction::E | Direction::W => vec![Direction::N, Direction::S],
        },
        '-' => match direction {
            Direction::E | Direction::W => vec![direction],
            Direction::N | Direction::S => vec![Direction::E, Direction::W],
        },
        '/' => match direction {
            Direction::N => vec![Direction::E],
            Direction::S => vec![Direction::W],
            Direction::E => vec![Direction::N],
            Direction::W => vec![Direction::S],
        },
        '\\' => match direction {
            Direction::N => vec![Direction::W],
            Direction::S => vec![Direction::E],
            Direction::E => vec![Direction::S],
            Direction::W => vec![Direction::N],
        },
        _ => unreachable!("Unknown tile: {tile}")
    };

    for dir in next_directions {
        if let Some(cordinate) = is_in_bounds(cordinate, dir, grid) {
            walk(cordinate, dir, grid)
        }
    }
}

fn is_in_bounds(
    cordinate: Cordinate,
    direction: Direction,
    grid: &mut Grid
) -> Option<Cordinate> {
    let (x, y) = cordinate;
    match direction {
        Direction::N => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        Direction::S => {
            if y < grid.num_rows() - 1 {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Direction::W => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        Direction::E => {
            if x < grid.num_cols() - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 51);
    }
}
