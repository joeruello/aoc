use std::collections::HashSet;

use common::{Direction, Itertools};
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

fn turn_90(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let mut pos = find_start(&grid);
    let mut dir = Direction::N;
    let mut visisted = HashSet::from([pos]);
    loop {
        let (x, y) = pos;
        let next_pos = match dir {
            Direction::N => (y > 0).then_some((x, y.saturating_sub(1))),
            Direction::S => (y < grid.num_rows() - 1).then_some((x, y + 1)),
            Direction::W => (x > 0).then_some((x.saturating_sub(1), y)),
            Direction::E => (x < grid.num_cols() - 1).then_some((x + 1, y)),
        };

        if let Some(next_pos) = next_pos {
            if grid[next_pos] == '#' {
                dir = turn_90(dir);
            } else {
                pos = next_pos;
                visisted.insert(pos);
            }
        } else {
            break;
        }
    }

    visisted.len()
}

fn find_start(grid: &TooDee<char>) -> (usize, usize) {
    let (width, height) = grid.size();

    for p in (0..width).cartesian_product(0..height) {
        if grid[p] == '^' {
            return p;
        }
    }
    panic!("couldn't find start");
}
