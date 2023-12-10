use std::{error::Error, str::FromStr};

use itertools::Itertools;
use num::Integer;

#[derive(Debug, PartialEq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

type Point = (usize, usize);

impl Tile {
    fn directions(&self) -> (Direction, Direction) {
        use Direction::*;
        match &self {
            Tile::NorthSouth => (North, South),
            Tile::EastWest => (East, West),
            Tile::NorthEast => (North, East),
            Tile::NorthWest => (North, West),
            Tile::SouthWest => (South, West),
            Tile::SouthEast => (South, East),
            Tile::Ground => panic!("ground isnt valid"),
            Tile::Start => panic!("start isnt valid"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, (x, y): Point) -> Point {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl FromStr for Tile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::NorthSouth),
            "-" => Ok(Self::EastWest),
            "L" => Ok(Self::NorthEast),
            "J" => Ok(Self::NorthWest),
            "7" => Ok(Self::SouthWest),
            "F" => Ok(Self::SouthEast),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => panic!("Unknown tile {s}"),
        }
    }
}

fn neighbours(map: &[Vec<Tile>], (x, y): (usize, usize)) -> Vec<Direction> {
    let north = if y > 1 {
        map.get(y - 1).and_then(|row| row.get(x))
    } else {
        None
    };
    let south = map.get(y + 1).and_then(|row| row.get(x));
    let east = map.get(y).and_then(|row| row.get(x + 1));
    let west = if x > 1 {
        map.get(y).and_then(|row| row.get(x - 1))
    } else {
        None
    };

    vec![
        (Direction::North, north),
        (Direction::East, east),
        (Direction::South, south),
        (Direction::West, west),
    ]
    .into_iter()
    .filter(|(_, opt)| opt.is_some_and(|t| *t != Tile::Ground))
    .map(|(dir, _)| dir)
    .collect()
}

fn get_tile(map: &[Vec<Tile>], (x, y): (usize, usize)) -> &Tile {
    map.get(y).and_then(|row| row.get(x)).unwrap()
}

fn find_start(map: &[Vec<Tile>]) -> Point {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .find_position(|tile| **tile == Tile::Start)
                .map(|(x, _)| (x, y))
        })
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().as_str().parse().unwrap())
                .collect()
        })
        .collect();

    find_pipe_length(&map)
}

fn find_pipe_length(map: &[Vec<Tile>]) -> usize {
    let start = find_start(map);
    let first_direction: Direction = *neighbours(map, start).first().unwrap();
    let mut walker = (first_direction.step(start), first_direction);

    for n in 1.. {
        let (point, prev_dir) = walker;
        let tile = get_tile(map, point);
        let next_walker = match tile {
            Tile::Start => (point, prev_dir),
            _ => {
                let (a, b) = tile.directions();
                let next_dir = if a.opposite() == prev_dir { b } else { a };
                let new_point = next_dir.step(point);
                println!("{point:?} moving {next_dir:?} to {new_point:?}");
                (new_point, next_dir)
            }
        };

        if matches!(get_tile(map, next_walker.0), Tile::Start) {
            return n.div_ceil(&2);
        }

        walker = next_walker;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 4);
    }

    #[test]
    fn test_sample2() {
        assert_eq!(process(include_str!("./sample2.txt")), 8);
    }
}
