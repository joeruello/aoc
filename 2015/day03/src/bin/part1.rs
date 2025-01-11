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

    let mut current = (0, 0);
    let mut visited = HashSet::<Cordinate>::new();
    visited.insert(current);
    for direction in directions {
        current = progress(current, direction);
        visited.insert(current);
    }
    visited.len()
}

fn progress((x, y): Cordinate, dir: Direction) -> Cordinate {
    match dir {
        Direction::N => (x, y + 1),
        Direction::S => (x, y - 1),
        Direction::E => (x + 1, y),
        Direction::W => (x - 1, y),
        _ => panic!("Unsupported Dir"),
    }
}
