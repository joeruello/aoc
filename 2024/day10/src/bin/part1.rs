use std::collections::{HashSet, VecDeque};

use common::Itertools;
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 10).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap_or(999))
            .collect_vec(),
    );

    (0..width)
        .cartesian_product(0..height)
        .filter(|p| grid[*p] == 0)
        .map(|p| score_trailhead(&grid, p))
        .sum()
}

fn score_trailhead(grid: &TooDee<u32>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::<(usize, usize)>::from([start]);
    let mut nines = HashSet::new();
    while let Some(point) = queue.pop_front() {
        if grid[point] == 9 {
            nines.insert(point);
            continue;
        }
        queue.extend(neighbours(point, grid).filter(|n| grid[*n] == grid[point] + 1));
    }
    nines.len()
}

fn neighbours((x, y): (usize, usize), grid: &TooDee<u32>) -> impl Iterator<Item = (usize, usize)> {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    [n, s, e, w].into_iter().flatten()
}
