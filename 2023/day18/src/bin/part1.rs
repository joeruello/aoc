use std::{error::Error, fmt::Debug, str::FromStr};

use itertools::Itertools;
use toodee::{TooDee, TooDeeOps};
type Cordinate = (usize, usize);
type Grid = TooDee<Tile>;
fn main() {
    let input: String = common::AocInput::fetch(2023, 18).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Clone, PartialEq, Eq, Hash, Default)]
enum Tile {
    #[default]
    Ground,
    Edge,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            // Tile::Hole => write!(f, "0"),
            Tile::Edge => write!(f, "#"),
        }
    }
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Direction::E,
            "D" => Direction::S,
            "L" => Direction::W,
            "U" => Direction::N,
            _ => panic!(),
        })
    }
}

fn walk((x, y): Cordinate, dir: Direction, grid: &Grid) -> Option<Cordinate> {
    match dir {
        Direction::N => (y > 0).then_some((x, y.saturating_sub(1))),
        Direction::S => (y < grid.num_rows() - 1).then_some((x, y + 1)),
        Direction::W => (x > 0).then_some((x.saturating_sub(1), y)),
        Direction::E => (x < grid.num_cols() - 1).then_some((x + 1, y)),
    }
}

type Instruction = (Direction, usize);

fn process(input: &str) -> usize {
    let instuctions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let (dir, count, _) = l.split_whitespace().collect_tuple().unwrap();
            (
                dir.parse().unwrap(),
                count.parse().unwrap()
            )
        })
        .collect();

    let width: usize = instuctions
        .iter()
        .filter_map(|(d, count)| (*d == Direction::E).then_some(count))
        .sum();
    let height: usize = instuctions
        .iter()
        .filter_map(|(d, count)| (*d == Direction::S).then_some(count))
        .sum();

    dbg!(width / 2, height / 2);
    let mut pointer: Cordinate = (width/2, height/2);
    let mut grid = TooDee::<Tile>::new(1000, 1000);

    for (direction, steps) in instuctions {
        for _ in 0..steps {
            grid[pointer] = Tile::Edge;
            if let Some(new_cordinate) = walk(pointer, direction, &grid) {
                pointer = new_cordinate;
            } else {
                panic!("Out of bounds! {pointer:?} {direction:?}")
            }
        }
    }

    grid.rows()
        .enumerate()
        .map(|(y, row)| {
            let mut crossings = 0;
            let mut holes = 0;
            let mut edges = 0;
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Ground => {
                        if crossings % 2 == 1 {
                            holes += 1;
                        }
                    }
                    Tile::Edge => {
                        if walk((x, y), Direction::N, &grid)
                            .is_some_and(|cord| grid[cord] == Tile::Edge)
                        {
                            crossings += 1
                        }
                        edges += 1;
                    }
                }
            }

            println!("{}", edges + holes);
            edges + holes
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 62);
    }
}
