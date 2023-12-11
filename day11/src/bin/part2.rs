use grid::Grid;
use itertools::Itertools;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, 1_000_000));
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Space,
    Galaxy,
}

fn process(input: &str, expansion_factor: usize) -> usize {
    let grid = parse(input);
    let (empty_rows, empty_cols) = expand(&grid);

    let galaxies: Vec<_> = grid
        .indexed_iter()
        .filter_map(|(index, tile)| {
            if *tile == Tile::Galaxy {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((y0, x0), (y1, x1))| {
            let dist_y = (min(y0, y1)..max(y0, y1)).fold(0, |mut acc, y| {
                if empty_rows.contains(&y) {
                    acc += expansion_factor
                } else {
                    acc += 1
                }
                acc
            });

            let dist_x = (min(x0, x1)..max(x0, x1)).fold(0, |mut acc, x| {
                if empty_cols.contains(&x) {
                    acc += expansion_factor
                } else {
                    acc += 1
                }
                acc
            });
            dist_x + dist_y
        })
        .sum()
}

fn parse(input: &str) -> Grid<Tile> {
    let cols = input.lines().next().unwrap().len();
    Grid::from_vec(
        input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::Space),
                '#' => Some(Tile::Galaxy),
                _ => None,
            })
            .collect(),
        cols,
    )
}

fn expand(grid: &Grid<Tile>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = grid
        .iter_rows()
        .enumerate()
        .filter_map(|(i, mut r)| {
            if r.all(|t| *t == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let empty_cols = grid
        .iter_cols()
        .enumerate()
        .filter_map(|(i, mut c)| {
            if c.all(|t| *t == Tile::Space) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    (empty_rows, empty_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt"), 2), 374);
        assert_eq!(process(include_str!("./sample.txt"), 10), 1030);
        assert_eq!(process(include_str!("./sample.txt"), 100), 8410);
    }
}
