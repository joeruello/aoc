use std::collections::HashSet;

use common::Itertools;
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 4).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().collect_vec().len();

    let directions: Vec<[(isize, isize); 3]> = vec![
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, -1), (-2, -2), (-3, -3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(1, -1), (2, -2), (3, -3)],
    ];

    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| c.is_uppercase()).collect(),
    );

    let mut count = 0;

    let mut marked = HashSet::new();

    for (x, y) in (0..width).cartesian_product(0..height) {
        if grid[(x, y)] != 'X' {
            continue;
        }
        for [(x1, y1), (x2, y2), (x3, y3)] in &directions {
            let x3 = x as isize + x3;
            let y3 = y as isize + y3;

            // We only need to bounds check the "S" because if any of the others are
            // out of bounds it will be as well
            if x3 < 0 || y3 < 0 || x3 > (width as isize) - 1 || y3 > (height as isize) - 1 {
                continue;
            }

            let m = ((x as isize + x1) as usize, (y as isize + y1) as usize);
            let a = ((x as isize + x2) as usize, (y as isize + y2) as usize);
            let s = (x3 as usize, y3 as usize);
            if [grid[m], grid[a], grid[s]] == ['M', 'A', 'S'] {
                count += 1;
                marked.extend([(x, y), m, a, s])
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            match marked.get(&(x, y)) {
                Some(_) => print!("{}", grid[(x, y)]),
                None => print!("."),
            }
        }
        println!();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process(include_str!("./sample.txt")), 18);
    }
}
