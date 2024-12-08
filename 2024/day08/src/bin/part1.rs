use std::collections::{HashMap, HashSet};

use common::Itertools;
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 8).unwrap().into();
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

    let mut antena: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for point in (0..width).cartesian_product(0..height) {
        match grid[point] {
            '.' => continue,
            c => antena
                .entry(c)
                .and_modify(|v| v.push(point))
                .or_insert(vec![point]),
        };
    }

    for a in antena.values() {
        for ((x1, y1), (x2, y2)) in a.iter().copied().tuple_combinations() {
            let dy = y1 as isize - y2 as isize;
            let dx = x1 as isize - x2 as isize;

            if let Some(p) = (x1, y1).add((dx, dy)) {
                if p.0 < width && p.1 < height {
                    antinodes.insert(p);
                }
            }

            if let Some(p) = (x2, y2).sub((dx, dy)) {
                if p.0 < width && p.1 < height {
                    antinodes.insert(p);
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if grid[(x, y)] != '.' {
                print!("{}", grid[(x, y)]);
            } else if antinodes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    antinodes.len()
}

trait PointOps: Sized {
    fn add(&self, d: (isize, isize)) -> Option<Self>;
    fn sub(&self, d: (isize, isize)) -> Option<Self>;
}

impl PointOps for (usize, usize) {
    fn add(&self, (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
        Some((
            self.0.checked_add_signed(dx)?,
            self.1.checked_add_signed(dy)?,
        ))
    }

    fn sub(&self, (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
        Some((
            self.0.checked_add_signed(-dx)?,
            self.1.checked_add_signed(-dy)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process(include_str!("./sample.txt")), 3);
    }
}
