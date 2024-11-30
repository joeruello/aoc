use std::{cmp::max, error::Error, str::FromStr};

#[derive(Debug, Default)]
struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn combine(self, rhs: &Self) -> Self {
        Self {
            red: max(self.red, rhs.red),
            blue: max(self.blue, rhs.blue),
            green: max(self.green, rhs.green),
        }
    }
}

impl FromStr for CubeSet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for part in s.split(',') {
            let num: u32 = part
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .expect("Expect value number");

            if part.ends_with("red") {
                red = num
            } else if part.ends_with("blue") {
                blue = num
            } else if part.ends_with("green") {
                green = num
            } else {
                panic!("Unknown color ${s}")
            }
        }
        Ok(Self::new(red, green, blue))
    }
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<CubeSet>,
}

impl Game {
    fn is_valid_superset(&self, set: CubeSet) -> bool {
        let total_set = self
            .sets
            .iter()
            .fold(CubeSet::default(), |set, item| set.combine(item));

        set.red >= total_set.red && set.blue >= total_set.blue && set.green >= total_set.green
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, cubes) = s.split_once(':').expect("Well formed input");
        let id = &game[5..];
        let cubes: Vec<CubeSet> = cubes
            .replace(' ', "")
            .split(';')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self {
            id: id.parse().expect("valid number"),
            sets: cubes,
        })
    }
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<Game>().expect("Well formed input"))
        .filter_map(|g| {
            g.is_valid_superset(CubeSet::new(12, 13, 14))
                .then_some(g.id)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 8);
    }
}
