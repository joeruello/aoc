use std::{cmp::max, error::Error, str::FromStr};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

#[derive(Debug, Default)]
struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    fn combine(self, rhs: &Self) -> Self {
        Self {
            red: max(self.red, rhs.red),
            blue: max(self.blue, rhs.blue),
            green: max(self.green, rhs.green),
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

#[derive(Debug)]
struct Game {
    pub sets: Vec<CubeSet>,
}

impl Game {
    fn min_set(&self) -> CubeSet {
        self.sets
            .iter()
            .fold(CubeSet::default(), |set, item| set.combine(item))
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
        Ok(Self { red, green, blue })
    }
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, cubes) = l.split_once(':').expect("Well formed input");
            let cubes = cubes.replace(' ', "");
            let cubes: Vec<CubeSet> = cubes.split(';').map(|s| s.parse().unwrap()).collect();
            Game { sets: cubes }
        })
        .map(|g| g.min_set().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 2286)
    }
}
