use std::collections::HashSet;
use toodee::{TooDee, TooDeeOps};

type Grid = TooDee<char>;
type Cordinate = (usize, usize);

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, 64));
}

fn process(input: &str, steps: usize) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    let mut positions: HashSet<Cordinate> = HashSet::from([find_start(&grid)]);
    for _ in 0..steps {
        let mut next = HashSet::new();
        for cord in positions.into_iter() {
            next.extend(neighbours(cord, &grid));
        }
        positions = next;
    }
    positions.len()
}

fn neighbours((x, y): Cordinate, grid: &Grid) -> Vec<Cordinate> {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    [n, s, e, w]
        .into_iter()
        .filter_map(|cord| cord.filter(|c| grid[*c] != '#'))
        .collect()
}

fn find_start(grid: &Grid) -> Cordinate {
    for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            if grid[(x, y)] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("Can't find start tile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_one_step() {
        assert_eq!(process(include_str!("./sample.txt"), 1), 2);
    }

    #[test]
    fn test_sample_two_step() {
        assert_eq!(process(include_str!("./sample.txt"), 2), 4);
    }

    #[test]
    fn test_sample_six_step() {
        assert_eq!(process(include_str!("./sample.txt"), 6), 16);
    }
}
