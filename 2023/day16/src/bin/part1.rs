use std::collections::HashSet;

use toodee::{TooDee, TooDeeOps};
type Cordinate = (usize, usize);
type Grid = TooDee<(char, HashSet<Direction>)>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let mut grid = TooDee::from_vec(
        width,
        height,
        input
            .replace('\n', "")
            .trim()
            .chars()
            .map(|c| (c, HashSet::<Direction>::new()))
            .collect(),
    );

    walk((0, 0), Direction::E, &mut grid);
    count_energised_cells(&grid)
}

fn count_energised_cells(grid: &Grid) -> usize {
    grid.cells().filter(|(_, set)| !set.is_empty()).count()
}

fn walk(cordinate: Cordinate, direction: Direction, grid: &mut Grid) {
    let (tile, visited_directions) = &mut grid[cordinate];
    if visited_directions.contains(&direction) {
        return;
    }

    visited_directions.insert(direction);
    let next_directions = match (tile, direction) {
        ('.', _) => vec![direction],
        ('|', Direction::N | Direction::S) => vec![direction],
        ('|', Direction::E | Direction::W) => vec![Direction::N, Direction::S],
        ('-', Direction::E | Direction::W) => vec![direction],
        ('-', Direction::N | Direction::S) => vec![Direction::E, Direction::W],
        ('/', Direction::N) => vec![Direction::E],
        ('/', Direction::S) => vec![Direction::W],
        ('/', Direction::E) => vec![Direction::N],
        ('/', Direction::W) => vec![Direction::S],
        ('\\', Direction::N) => vec![Direction::W],
        ('\\', Direction::S) => vec![Direction::E],
        ('\\', Direction::E) => vec![Direction::S],
        ('\\', Direction::W) => vec![Direction::N],
        _ => unreachable!("Unknown tile"),
    };

    for dir in next_directions {
        if let Some(cordinate) = is_in_bounds(cordinate, dir, grid) {
            walk(cordinate, dir, grid)
        }
    }
}

fn is_in_bounds((x, y): Cordinate, direction: Direction, grid: &Grid) -> Option<Cordinate> {
    match direction {
        Direction::N => (y > 0).then_some((x, y.saturating_sub(1))),
        Direction::S => (y < grid.num_rows() - 1).then_some((x, y + 1)),
        Direction::W => (x > 0).then_some((x.saturating_sub(1), y)),
        Direction::E => (x < grid.num_cols() - 1).then_some((x + 1, y)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 46);
    }
}
