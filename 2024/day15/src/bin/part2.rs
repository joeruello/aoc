use std::collections::{HashMap, HashSet, VecDeque};

use common::{Direction, DirectionOps, Itertools, TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 15).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let width = grid.find("\n").unwrap() * 2;
    let height = grid.lines().count();
    let mut grid = TooDee::from_vec(
        width,
        height,
        grid.chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| match c {
                '#' => "##".chars(),
                'O' => "[]".chars(),
                '.' => "..".chars(),
                '@' => "@.".chars(),
                _ => unreachable!("invaid tile {}", c),
            })
            .collect_vec(),
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

    'outer: for dir in instructions {
        let start = grid.find('@').expect("Should be in grid");
        let mut diff = HashMap::from([(start, '.')]);
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);
        while let Some(next) = queue
            .pop_front()
            .and_then(|p| grid.move_point(&p, dir.xy()))
        {
            if visited.contains(&next) {
                continue;
            }

            let prev = grid.move_point(&next, dir.rot180().xy()).unwrap();
            let char = if visited.contains(&prev) {
                grid[prev]
            } else {
                '.'
            };
            diff.insert(next, char);
            match grid[next] {
                '#' => {
                    // invalid move, discard all pending changes
                    continue 'outer;
                }
                '.' | '@' => {}
                '[' => {
                    queue.push_back(next);
                    if dir == Direction::N || dir == Direction::S {
                        queue.push_back((prev.0 + 1, prev.1));
                        queue.push_back((next.0 + 1, next.1));
                    }
                }
                ']' => {
                    queue.push_back(next);
                    if dir == Direction::N || dir == Direction::S {
                        queue.push_back((prev.0 - 1, prev.1));
                        queue.push_back((next.0 - 1, next.1));
                    }
                }
                _ => unreachable!("invalid tile: {}", grid[next]),
            };

            visited.insert(next);
        }
        // nove is valid, apply changes to state
        for (point, tile) in diff {
            grid[point] = tile;
        }
    }

    let mut sum = 0;
    for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            let tile = grid[(x, y)];
            print!("{}", tile);
            if tile == '[' {
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
        assert_eq!(process(include_str!("./sample3.txt")), 618);
        assert_eq!(process(include_str!("./sample2.txt")), 9021);
    }
}
