use std::collections::HashSet;

use common::{Direction as Dir, Itertools};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let start = find_start(&grid);
    let mut pos = start;
    let mut dir = Dir::N;
    let mut visited = HashSet::from([pos]);

    loop {
        let next_pos = next_position(dir, pos, &grid);
        if let Some(next_pos) = next_pos {
            if grid[next_pos] == '#' {
                dir = turn_90(dir);
            } else {
                pos = next_pos;
                visited.insert(pos);
            }
        } else {
            break;
        }
    }

    visited
        .into_par_iter()
        .filter(|p| {
            if *p == start {
                return false;
            }
            let mut pos = start;
            let mut dir = Dir::N;
            let mut visited = HashSet::from([(pos, dir)]);
            loop {
                let next_pos = next_position(dir, pos, &grid);
                if let Some(next_pos) = next_pos {
                    if visited.contains(&(next_pos, dir)) {
                        return true;
                    } else if next_pos == *p || grid[next_pos] == '#' {
                        dir = turn_90(dir);
                    } else {
                        pos = next_pos;
                    }
                    visited.insert((pos, dir));
                } else {
                    return false;
                }
            }
        })
        .count()
}

fn next_position(dir: Dir, (x, y): (usize, usize), grid: &TooDee<char>) -> Option<(usize, usize)> {
    match dir {
        Dir::N => (y > 0).then_some((x, y.saturating_sub(1))),
        Dir::S => (y < grid.num_rows() - 1).then_some((x, y + 1)),
        Dir::W => (x > 0).then_some((x.saturating_sub(1), y)),
        Dir::E => (x < grid.num_cols() - 1).then_some((x + 1, y)),
    }
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

fn turn_90(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}
