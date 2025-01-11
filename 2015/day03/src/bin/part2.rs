use common::Direction;
use core::panic;
use std::collections::HashSet;

fn main() {
    let input: String = common::AocInput::fetch(2015, 3).unwrap().into();
    println!("Output: {}", process(&input));
}

type Cordinate = (isize, isize);

fn process(input: &str) -> usize {
    let directions: Vec<_> = input
        .chars()
        .map(|c| match c {
            '^' => Direction::N,
            '<' => Direction::W,
            '>' => Direction::E,
            'v' => Direction::S,
            _ => panic!("Unknown direction {c}"),
        })
        .collect();

    let mut visited = HashSet::<Cordinate>::new();
    let mut santa = (0, 0);
    let mut robot_santa = (0, 0);
    visited.insert(santa);
    for (i, direction) in directions.into_iter().enumerate() {
        if i % 2 == 0 {
            santa = progress(santa, direction);
            visited.insert(santa);
        } else {
            robot_santa = progress(robot_santa, direction);
            visited.insert(robot_santa);
        }
    }
    visited.len()
}

fn progress((x, y): Cordinate, dir: Direction) -> Cordinate {
    match dir {
        Direction::N => (x, y + 1),
        Direction::S => (x, y - 1),
        Direction::E => (x + 1, y),
        Direction::W => (x - 1, y),
        _ => panic!("Unknown direction"),
    }
}
