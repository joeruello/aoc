use std::{error::Error, str::FromStr, vec};

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
    assert!(neighbours.len() == 2);
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
        (Direction::South, south),
        (Direction::East, east),
        (Direction::West, west),
    ]
    .into_iter()
    .filter(|(dir, opt)| {
        opt.is_some_and(|t| {
            if *t == Tile::Ground {
                return false;
            }
            t.directions().0 == dir.opposite() || t.directions().1 == dir.opposite()
        })
    })
    .map(|(dir, _)| dir)
    .collect()
}

fn find_main_loop(map: &[Vec<Tile>]) -> Vec<Point> {
    let start = find_start(map);
    let first_direction: Direction = *neighbours(map, start).first().unwrap();
    let mut walker = (first_direction.step(start), first_direction);
    let mut main_loop = vec![start, walker.0];
    loop {
        let (point, prev_dir) = walker;
        let tile = get_tile(map, point);
        let next_walker = match tile {
            Tile::Start => (point, prev_dir),
            _ => {
                let (a, b) = tile.directions();
                let next_dir = if a.opposite() == prev_dir { b } else { a };
                let new_point = next_dir.step(point);
                main_loop.push(new_point);
                (new_point, next_dir)
            }
        };

        if matches!(get_tile(map, next_walker.0), Tile::Start) {
            return main_loop;
        }

        walker = next_walker;
    }
}

fn get_tile(map: &[Vec<Tile>], (x, y): (usize, usize)) -> &Tile {
    map.get(y).and_then(|row| row.get(x)).unwrap()
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

    let width = map.first().map(|f| f.len()).unwrap();
    let height = map.len();

    let main_loop = find_main_loop(&map);
    let mut width_values: Vec<Vec<_>> = vec![];
    for y in 0..height {
        let mut winding_number = 0;
        width_values.push(
            (0..width)
                .map(|x| match main_loop.contains(&(x, y)) {
                    true => {
                        let tile = get_tile(&map, (x, y));
                        if *tile == Tile::Start {
                            winding_number += dbg!(determine_start_type(&map)).winding_number()
                        } else {
                            winding_number += tile.winding_number();
                        }
                        false
                    }
                    false => winding_number.is_odd(),
                })
                .collect(),
        )
    }

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            match main_loop.contains(&(x, y)) {
                false => {
                    if width_values[y][x] {
                        print!("*");
                        count += 1;
                    } else {
                        print!(" ")
                    }
                }
                true => {
                    print!("█")
                }
            }
        }
        println!();
    }

    count
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
