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

    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| c.is_uppercase()).collect(),
    );

    let mut count = 0;

    let mut marked = HashSet::new();

    for (x, y) in (1..width - 1).cartesian_product(1..height - 1) {
        if grid[(x, y)] != 'A' {
            continue;
        }
        let a = (x + 1, y + 1);
        let b = (x - 1, y - 1);
        let c = (x - 1, y + 1);
        let d = (x + 1, y - 1);

        if grid[a] == grid[b] || grid[c] == grid[d] {
            continue;
        }

        let hist = [grid[a], grid[b], grid[c], grid[d]].into_iter().counts();
        let has_two_ss = *hist.get(&'S').unwrap_or(&0) == 2;
        let has_two_ms = *hist.get(&'M').unwrap_or(&0) == 2;
        if has_two_ms && has_two_ss {
            count += 1;
            marked.extend([(x, y), a, b, c, d]);
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
        println!();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            process(
                "XMX
MAS
XSX"
            ),
            0
        );

        assert_eq!(
            process(
                "XMX
SAS
XMX"
            ),
            0
        );

        assert_eq!(
            process(
                "MXM
XAX
SXS"
            ),
            1
        );

        assert_eq!(
            process(
                "MXS
XAX
SXM"
            ),
            0
        );

        assert_eq!(process(include_str!("./sample.txt")), 9);
    }
}
