use std::{error::Error, fmt::Debug, str::FromStr};

use indicatif::ProgressIterator;
use itertools::Itertools;
type Cordinate = (isize, isize);
fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
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

fn walk((x, y): Cordinate, dir: Direction, distance: usize) -> Cordinate {
    match dir {
        Direction::N => (x, y - distance as isize),
        Direction::S => (x, y + distance as isize),
        Direction::W => (x - distance as isize, y),
        Direction::E => (x + distance as isize, y),
    }
}

type Instruction = (Direction, usize);

fn process(input: &str) -> usize {
    let instuctions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let (_, _, hex) = l.split_whitespace().collect_tuple().unwrap();
            let hex = hex.replace(['(', ')'], "");
            let distance = usize::from_str_radix(&hex[1..hex.len() - 1], 16).unwrap();
            let dir = match &hex[hex.len() - 1..] {
                "0" => Direction::E,
                "1" => Direction::S,
                "2" => Direction::W,
                "3" => Direction::N,
                _ => panic!("Unknown dir"),
            };

            (dir, distance)
        })
        .collect();

    let mut pointer: Cordinate = (0, 0);
    let mut points: Vec<Cordinate> = vec![pointer];
    let mut perimeter = 0;

    for (direction, distance) in instuctions.into_iter().progress() {
        pointer = walk(pointer, direction, distance);
        perimeter += distance;
        points.push(pointer);
    }

    let internal_area = shoelace(points) as usize;

    internal_area + perimeter/2 + 1
}

fn shoelace(points: Vec<Cordinate>) -> isize {
    points.into_iter().tuple_windows().map(|(a,b)| det(a, b)).sum::<isize>() / 2
}

fn det((x1,y1): Cordinate, (x2,y2): Cordinate) -> isize {
    x1 * y2 - y1 * x2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 952408144115);
    }
}
