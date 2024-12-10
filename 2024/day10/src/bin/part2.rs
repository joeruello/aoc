use std::collections::VecDeque;

use common::Itertools;
use toodee::{TooDee, TooDeeOps};

type Grid = TooDee<u32>;
type Point = (usize, usize);

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

fn score_trailhead(grid: &Grid, start: Point) -> usize {
    let mut queue = VecDeque::<Point>::from([start]);
    let mut count = 0;
    while let Some(point) = queue.pop_front() {
        if grid[point] == 9 {
            count += 1;
            continue;
        }
        queue.extend(neighbours(point, grid).filter(|n| grid[*n] == grid[point] + 1));
    }
    count
}

fn neighbours((x, y): Point, grid: &Grid) -> impl Iterator<Item = Point> {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    [n, s, e, w].into_iter().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample2.txt")), 2);
        assert_eq!(process(include_str!("./sample3.txt")), 4);
        assert_eq!(process(include_str!("./sample.txt")), 36);
    }
}
