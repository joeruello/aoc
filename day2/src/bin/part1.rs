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
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<CubeSet>,
}

impl Game {
    fn is_set_possible(&self, set: CubeSet) -> bool {
        let total_set = self
            .sets
            .iter()
            .fold(CubeSet::default(), |set, item| set.combine(item));

        set.red >= total_set.red && set.blue >= total_set.blue && set.green >= total_set.green
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
            // dbg!(num)

            if part.ends_with("red") {
                red = num
            } else if part.ends_with("blue") {
                blue = num
            } else if part.ends_with("green") {
                green = num
            } else {
                panic!("Unknown color ${s}")
            }

            dbg!(s, red, blue, green);
        }
        Ok(Self { red, green, blue })
    }
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (game, cubes) = l.split_once(':').expect("Well formed input");
            let id = &game[5..];
            let cubes = cubes.replace(' ', "");
            let cubes: Vec<CubeSet> = cubes.split(';').map(|s| s.parse().unwrap()).collect();
            Game {
                id: id.parse().expect("valid number"),
                sets: cubes,
            }
        })
        .filter_map(|g| {
            if g.is_set_possible(CubeSet {
                red: 12,
                green: 13,
                blue: 14,
            }) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 8)
    }
}
