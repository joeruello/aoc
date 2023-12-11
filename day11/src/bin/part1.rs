use grid::Grid;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Space,
    Galaxy,
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

fn process(input: &str) -> usize {
    let grid = parse(input);
    let grid = expand(grid);

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
        .map(|((y0, x0), (y1, x1))| y0.abs_diff(y1) + x0.abs_diff(x1))
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

fn expand(mut grid: Grid<Tile>) -> Grid<Tile> {
    let empty_rows: Vec<_> = grid
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

    let empty_cols: Vec<_> = grid
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

    for (i, row) in empty_rows.into_iter().enumerate() {
        grid.insert_row(row + i, [Tile::Space].repeat(grid.cols()));
    }

    for (i, col) in empty_cols.into_iter().enumerate() {
        grid.insert_col(col + i, [Tile::Space].repeat(grid.rows()))
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let grid = expand(parse(include_str!("./sample.txt")));
        let expanded = parse(include_str!("./expanded-sample.txt"));
        assert_eq!(grid.flatten(), expanded.flatten());
    }

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 374);
    }
}
