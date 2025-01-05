use std::collections::{HashMap, VecDeque};

use common::{Direction, DirectionOps, Itertools,TooDee};

fn main() {
    let input: String = common::AocInput::fetch(2024, 20).unwrap().into();
    println!("Output: {}", process(&input, 100));
}

fn process(input: &str, pico_threshold: usize) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let start = grid.find('S').unwrap();

    let mut queue = VecDeque::from([(start, 0usize)]);
    let mut positions = HashMap::new();

    while let Some((point, time)) = queue.pop_front() {
        if positions.contains_key(&point) {
            continue;
        }

        positions.insert(point, time);
        if grid[point] == 'E' {
            break;
        }
        for dir in Direction::CARDINALS {
            if let Some(next) = grid.move_point(&point, dir) {
                if grid[next] != '#' {
                    queue.push_back((next, time + 1));
                }
            }
        }
    }

    let mut cheats = HashMap::new();

    for ((a, ta), (b, tb)) in positions.iter().tuple_combinations() {
        if distance(a, b) == 2 {
            let diff = tb.abs_diff(*ta);
            if diff > 2 {
                let saved = diff - 2;
                cheats.insert((a, b), saved);
            }
        }
    }

    cheats
        .into_iter()
        .filter(|(_, v)| *v >= pico_threshold)
        .count()
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 64), 1);
    }
}
