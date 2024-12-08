use std::collections::{HashMap, HashSet};

use common::Itertools;
use toodee::{TooDee, TooDeeOps};

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

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for point in (0..width).cartesian_product(0..height) {
        match grid[point] {
            '.' => continue,
            c => antennas
                .entry(c)
                .and_modify(|v| v.push(point))
                .or_insert(vec![point]),
        };
    }

    for a in antennas.values() {
        for ((x1, y1), (x2, y2)) in a.iter().copied().tuple_combinations() {
            let dy = y1 as isize - y2 as isize;
            let dx = x1 as isize - x2 as isize;

            if let Some(np) = find_point((x1, y1), (dx, dy), grid.size()) {
                antinodes.insert(np);
            }

            if let Some(np) = find_point((x2, y2), (-dx, -dy), grid.size()) {
                antinodes.insert(np);
            }
        }
    }

    antinodes.len()
}

fn find_point(
    (x0, y0): (usize, usize),
    (dx, dy): (isize, isize),
    (width, height): (usize, usize),
) -> Option<(usize, usize)> {
    let x = x0.checked_add_signed(dx)?;
    let y = y0.checked_add_signed(dy)?;
    (x < width && y < height).then_some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process(include_str!("./sample.txt")), 4);
    }
}
