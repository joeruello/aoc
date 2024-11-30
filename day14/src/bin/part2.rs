use std::collections::HashMap;
use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let mut grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    let mut grid_cache: HashMap<TooDee<char>, (TooDee<char>, usize)> = HashMap::new();
    let mut target = None;
    for i in 0..1000000000 {
        if let Some((entry, idx)) = grid_cache.get(&grid) {
            let cycle_len = i - idx;
            let cycle_start = idx;
            let target_idx = (1000000000 - cycle_start) % cycle_len;
            target = Some(i + target_idx - 1);
            grid = entry.clone();
        } else {
            let start_grid = grid.clone();
            tilt_north(&mut grid);
            tilt_west(&mut grid);
            tilt_south(&mut grid);
            tilt_east(&mut grid);
            grid_cache.insert(start_grid, (grid.clone(), i));
        }

        if target.is_some_and(|t| t == i) {
            break;
        }
    }

    let mut sum = 0;
    for (i, row) in grid.rows().enumerate() {
        let count = row.iter().filter(|c| **c == 'O').count();
        sum += count * (grid.num_rows() - i)
    }

    sum
}

fn tilt_north(grid: &mut TooDee<char>) {
    for x in 0..grid.num_cols() {
        let col: Vec<char> = grid.col(x).cloned().collect();
        let new_col = collapse(&col);
        for (i, c) in grid.col_mut(x).enumerate() {
            *c = new_col[i];
        }
    }
}

fn tilt_south(grid: &mut TooDee<char>) {
    for x in 0..grid.num_cols() {
        let col: Vec<char> = grid.col(x).rev().cloned().collect();
        let new_col = collapse(&col);
        for (i, c) in grid.col_mut(x).rev().enumerate() {
            *c = new_col[i];
        }
    }
}

fn tilt_west(grid: &mut TooDee<char>) {
    let num_cols = grid.num_cols();
    for y in 0..grid.num_rows() {
        let col: Vec<char> = grid[y].to_vec();
        let new_row = collapse(&col);
        grid[y][..num_cols].copy_from_slice(&new_row[..num_cols]);
    }
}

fn tilt_east(grid: &mut TooDee<char>) {
    let num_cols = grid.num_cols();
    for y in 0..grid.num_rows() {
        let col: Vec<char> = grid[y].iter().rev().cloned().collect();
        let mut new_row = collapse(&col);
        new_row.reverse();
        grid[y][..num_cols].copy_from_slice(&new_row[..num_cols]);
    }
}

fn collapse(pattern: &[char]) -> Vec<char> {
    let chunks: Vec<_> = pattern.split_inclusive(|c| *c == '#').collect();
    chunks
        .into_iter()
        .flat_map(|chunk| {
            let count = chunk.iter().filter(|c| **c == 'O').count();
            let mut ret: Vec<_> = ['O']
                .repeat(count)
                .into_iter()
                .chain(['.'].repeat(chunk.len() - count))
                .collect();
            if *chunk.last().unwrap() == '#' {
                ret[chunk.len() - 1] = '#';
            }
            ret
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 64);
    }
}
