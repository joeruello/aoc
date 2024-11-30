use std::{error::Error, str::FromStr, vec};

use itertools::Itertools;
use num::Integer;

type Point = (usize, usize);

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

    fn winding_number(&self) -> isize {
        match self {
            Tile::NorthSouth => 1,
            Tile::EastWest => 0,
            Tile::NorthEast => 1,
            Tile::NorthWest => 1,
            Tile::SouthWest => 0,
            Tile::SouthEast => 0,
            Tile::Ground => 0,
            Tile::Start => 0,
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
    fn progress_in_direction(&self, (x, y): Point) -> Point {
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

    fn is_opposite(&self, other: &Direction) -> bool {
        self.opposite() == *other
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

fn determine_start_type(map: &[Vec<Tile>]) -> Tile {
    let start = find_start(map);
    let neighbours = neighbours(map, start);
    use Direction::*;
    match (neighbours.first().unwrap(), neighbours.last().unwrap()) {
        (North, South) => Tile::NorthSouth,
        (East, West) => Tile::EastWest,
        (North, East) => Tile::NorthEast,
        (North, West) => Tile::NorthWest,
        (South, West) => Tile::SouthWest,
        (South, East) => Tile::SouthEast,
        _ => panic!("unknown direction"),
    }
}

fn neighbours(map: &[Vec<Tile>], (x, y): (usize, usize)) -> Vec<Direction> {
    vec![
        (
            Direction::North,
            if y > 1 {
                map.get(y - 1).and_then(|row| row.get(x))
            } else {
                None
            },
        ),
        (Direction::South, map.get(y + 1).and_then(|row| row.get(x))),
        (Direction::East, map.get(y).and_then(|row| row.get(x + 1))),
        (
            Direction::West,
            if x > 1 {
                map.get(y).and_then(|row| row.get(x - 1))
            } else {
                None
            },
        ),
    ]
    .into_iter()
    .filter(|(dir, opt)| {
        opt.is_some_and(|t| {
            if *t == Tile::Ground {
                return false;
            }
            dir.is_opposite(&t.directions().0) || dir.is_opposite(&t.directions().1)
        })
    })
    .map(|(dir, _)| dir)
    .collect()
}

fn find_main_loop(map: &[Vec<Tile>], start: Point) -> Vec<Point> {
    let first_direction: Direction = *neighbours(map, start).first().unwrap();
    let mut walker = (
        first_direction.progress_in_direction(start),
        first_direction,
    );
    let mut main_loop = vec![start, walker.0];
    loop {
        let (point, prev_dir) = walker;
        let tile = get_tile(map, point);
        walker = match tile {
            Tile::Start => return main_loop,
            _ => {
                let (a, b) = tile.directions();
                let next_dir = if prev_dir.is_opposite(&a) { b } else { a };
                let new_point = next_dir.progress_in_direction(point);
                main_loop.push(new_point);
                (new_point, next_dir)
            }
        };
    }
}

fn get_tile(map: &[Vec<Tile>], (x, y): (usize, usize)) -> &Tile {
    map.get(y).and_then(|row| row.get(x)).unwrap()
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

    let width = map.first().map(|f| f.len()).unwrap();
    let height = map.len();

    let start = find_start(&map);
    let main_loop = find_main_loop(&map, start);
    let mut count = 0;
    for y in 0..height {
        let mut winding_number = 0;
        for x in 0..width {
            if main_loop.contains(&(x, y)) {
                let tile = get_tile(&map, (x, y));
                if *tile == Tile::Start {
                    winding_number += determine_start_type(&map).winding_number()
                } else {
                    winding_number += tile.winding_number();
                }
                print!("█");
            } else if winding_number.is_odd() {
                print!("*");
                count += 1
            } else {
                print!(" ");
            }
        }
        println!()
    }

    count
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 1);
    }

    #[test]
    fn test_sample3() {
        assert_eq!(process(include_str!("./sample3.txt")), 4);
    }

    #[test]
    fn test_sample4() {
        assert_eq!(process(include_str!("./sample4.txt")), 4);
    }

    #[test]
    fn test_sample5() {
        assert_eq!(process(include_str!("./sample5.txt")), 8);
    }

    #[test]
    fn test_sample6() {
        assert_eq!(process(include_str!("./sample6.txt")), 10);
    }
}
