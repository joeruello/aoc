use std::{collections::VecDeque, usize};

use common::{DirectionOps, TooDee};

fn main() {
    let input: String = common::AocInput::fetch(2025, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i32 {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let mut grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    let mut split_count = 0;
    let start = grid.find('S').expect("a start");
    let mut beams: VecDeque<(usize, usize)> = VecDeque::from_iter([start]);
    while let Some(beam) = beams.pop_front() {
        match grid[beam] {
            'S' | '.' => {
                if let Some(south) = grid.move_point(&beam, (1, 0)) {
                    grid[beam] = '|';
                    dbg!(&south);
                    beams.push_back(south);
                }
            }
            '^' => {
                if let Some(east) = grid.move_point(&beam, (0, -1)) {
                    beams.push_back(east);
                }
                if let Some(west) = grid.move_point(&beam, (0, 1)) {
                    beams.push_back(west);
                }
                split_count += 1;
            }
            _ => {
                dbg!(&beam, grid[beam]);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[(x, y)]);
        }
        println!();
    }

    split_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 21);
    }
}
