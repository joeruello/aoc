use std::collections::HashSet;

use toodee::{TooDee, TooDeeOps};

type Grid = TooDee<char>;
type Cordinate = (usize, usize);


fn neighbours((x, y): Cordinate, grid: &Grid) -> Vec<Cordinate> {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    vec![n, s, e, w]
        .into_iter()
        .filter_map(|cord| cord.filter(|c| grid[*c] != '#'))
        .collect()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, 64));
}

fn process(input: &str, num_steps: usize) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    let mut start = (usize::MAX, usize::MAX);

    'outer: for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            if grid[(x, y)] == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut visited = HashSet::new();
    let mut next: HashSet<Cordinate> = neighbours(start, &grid).into_iter().collect();

    for n in 0..num_steps {
        println!("Step {n}: Number in queue: {}", next.len());
        visited.clear();
        let mut next_set = vec![];
        for cord in next.into_iter() {
            visited.insert(cord);
            next_set.append(&mut neighbours(cord, &grid));
        }
        next = next_set.into_iter().collect();
    }


    for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            let tile = grid[(x,y)];
            print!("{}", match (tile, visited.contains(&(x,y))) {
                ('S'|'.', true) => '0',
                ('S'|'.', false) => tile,
                ('#', false) => '#',
                _ => panic!("Shouldnt be able to move into a wall at ({x},{y})")
            });
        }
        println!();
    }
    visited.len()

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
