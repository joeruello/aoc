use anyhow::{Context, Result};
use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 13).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> isize {
    input
        .split("\n\n")
        .map(|m| parse_machine(m).unwrap())
        .map(|m| cheapest_solve(m).unwrap_or(0))
        .sum()
}

fn cheapest_solve(m: Machine) -> Option<isize> {
    // https://en.wikipedia.org/wiki/Cramer%27s_rule
    let det = m.a.0 * m.b.1 - m.a.1 * m.b.0;
    let det_x = m.prize.0 * m.b.1 - m.prize.1 * m.b.0;

    let det_y = m.prize.1 * m.a.0 - m.prize.0 * m.a.1;

    if det_x % det != 0 || det_y % det != 0 {
        return None;
    }

    let a = det_x / det;
    let b = det_y / det;

    Some(a * 3 + b)
}

#[derive(Debug)]
struct Machine {
    pub a: (isize, isize),
    pub b: (isize, isize),
    pub prize: (isize, isize),
}

impl Machine {
    fn new(a: (isize, isize), b: (isize, isize), prize: (isize, isize)) -> Self {
        Self { a, b, prize }
    }
}

fn parse_machine(line: &str) -> Result<Machine> {
    let (a, b, prize) = line.lines().collect_tuple().context("")?;

    Ok(Machine::new(
        parse_coords(a, '+')?,
        parse_coords(b, '+')?,
        parse_coords(prize, '=')?,
    ))
}

fn parse_coords(l: &str, sep: char) -> Result<(isize, isize)> {
    let (_, a) = l.split_once(":").context("split0")?;
    let (xa, ya) = a.split_once(", ").context("split")?;
    let (_, xa) = xa.split_once(sep).context("split2")?;
    let xa = xa.parse::<isize>()?;
    let (_, ya) = ya.split_once(sep).context("split3")?;
    let ya = ya.parse::<isize>()?;
    Ok((xa, ya))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./1.txt")), 280);
    }
}
