use std::collections::VecDeque;

use common::{Direction, DirectionOps, Itertools};
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 15).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let width = grid.find("\n").unwrap();
    let height = grid.lines().count();
    let mut grid = TooDee::from_vec(
        width,
        height,
        grid.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let instructions = instructions
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Direction::N,
            '>' => Direction::E,
            'v' => Direction::S,
            '<' => Direction::W,
            _ => panic!("invalid direction direction: {c}"),
        })
        .collect_vec();

    let start = grid.find('@').expect("to be in grid");

    let mut current = start;
    'outer: for dir in instructions {
        if let Some(target) = grid.move_point(&current, dir.xy()) {
            let mut stack = VecDeque::from([(current, '.'), (target, '@')]);
            match grid[target] {
                '#' => continue,
                '.' => {}
                'O' => {
                    let mut pointer = target;
                    while let Some(next) = grid.move_point(&pointer, dir.xy()) {
                        match grid[next] {
                            '#' => continue 'outer,
                            '.' => {
                                stack.push_front((next, grid[pointer]));
                                break;
                            }
                            'O' => {
                                stack.push_front((next, grid[pointer]));
                                pointer = next;
                            }
                            _ => unreachable!("invalid tile: {}", grid[pointer]),
                        }
                    }
                }
                _ => unreachable!("invaid tile {}", grid[target]),
            }
            for (point, tile) in stack {
                grid[point] = tile;
            }

            current = target;
            if grid[target] == 'O' {}
        } else {
            continue;
        }
    }

    let mut sum = 0;
    for y in 0..grid.num_cols() {
        for x in 0..grid.num_rows() {
            let tile = grid[(x, y)];
            print!("{}", tile);
            if tile == 'O' {
                sum += x + y * 100;
            }
        }
        println!();
    }
    println!();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample2.txt")), 10092);
        assert_eq!(process(include_str!("./sample.txt")), 2028);
    }
}
