use std::collections::{HashSet, VecDeque};

use common::Itertools;
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2024, 12).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let mut sum = 0;

    let mut visited = HashSet::new();
    let mut regions = VecDeque::from([(0, 0)]);
    while let Some(region) = regions.pop_front() {
        let mut area = 0;
        let mut perim = 0;
        let mut current = VecDeque::from([region]);
        while let Some(p) = current.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            area += 1;
            for n in neighbours(p, &grid) {
                if let Some(n) = n {
                    if grid[n] == grid[p] {
                        current.push_back(n);
                    } else {
                        perim += 1;
                        if !visited.contains(&n) {
                            regions.push_back(n);
                        }
                    }
                } else {
                    perim += 1
                }
            }
            visited.insert(p);
        }
        println!("region of {} with price {area} x {perim}", grid[region]);
        sum += area * perim
    }

    sum
}

fn neighbours((x, y): (usize, usize), grid: &TooDee<char>) -> [Option<(usize, usize)>; 4] {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    [n, e, s, w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 1930);
    }
}
