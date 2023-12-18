use std::collections::HashSet;

use toodee::{TooDee, TooDeeOps};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
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

    let mut count = 0;
    for row in grid.rows() {
        for (col, set) in row {
            if !set.is_empty() {
                print!("#");
                count+=1;
            } else {
                print!(".")
            }
        }
        println!()
    }

    count
}

type Cordinate = (usize, usize);

fn walk(cordinate: Cordinate, direction: Direction, grid: &mut TooDee<(char, HashSet<Direction>)>) {
    let (tile, directions) = &mut grid[cordinate];
    if directions.contains(&direction) {
        return
    } else {
        directions.insert(direction);
    }


    match tile {
        '.' => {
            if let Some(cordinate) = is_in_bounds(cordinate, direction, grid) {
                walk(cordinate, direction, grid)
            }
        }
        '|' => match direction {
            Direction::N | Direction::S => {
                if let Some(cordinate) = is_in_bounds(cordinate, direction, grid) {
                    walk(cordinate, direction, grid)
                }
            }
            Direction::E | Direction::W => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::N, grid) {
                    walk(cordinate, Direction::N, grid)
                }
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::S, grid) {
                    walk(cordinate, Direction::S, grid)
                }
            }
        },
        '-' => match direction {
            Direction::E | Direction::W => {
                if let Some(cordinate) = is_in_bounds(cordinate, direction, grid) {
                    walk(cordinate, direction, grid)
                }
            }
            Direction::N | Direction::S => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::E, grid) {
                    walk(cordinate, Direction::E, grid)
                }
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::W, grid) {
                    walk(cordinate, Direction::W, grid)
                }
            }
        },
        '/' => match direction {
            Direction::N => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::E, grid) {
                    walk(cordinate, Direction::E, grid)
                }
            }
            Direction::S => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::W, grid) {
                    walk(cordinate, Direction::W, grid)
                }
            }
            Direction::E => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::N, grid) {
                    walk(cordinate, Direction::N, grid)
                }
            }
            Direction::W => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::S, grid) {
                    walk(cordinate, Direction::S, grid)
                }
            }
        },
        '\\' => match direction {
            Direction::N => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::W, grid) {
                    walk(cordinate, Direction::W, grid)
                }
            }
            Direction::S => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::E, grid) {
                    walk(cordinate, Direction::E, grid)
                }
            }
            Direction::E => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::S, grid) {
                    walk(cordinate, Direction::S, grid)
                }
            }
            Direction::W => {
                if let Some(cordinate) = is_in_bounds(cordinate, Direction::N, grid) {
                    walk(cordinate, Direction::N, grid)
                }
            }
        },
        _ => unimplemented!("{tile} unimplemented"),
    }
}

fn is_in_bounds(
    cordinate: Cordinate,
    direction: Direction,
    grid: &mut TooDee<(char, HashSet<Direction>)>
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

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 405);
    }
}
