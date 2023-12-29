use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    error::Error,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vec3(usize, usize, usize);

impl Vec3 {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
    fn z(&self) -> usize {
        self.2
    }
}

impl From<(usize, usize, usize)> for Vec3 {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Self(x, y, z)
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Brick(Vec3, Vec3, usize);

impl Brick {
    fn i(&self) -> usize {
        self.2
    }

    fn z0(&self) -> usize {
        min(self.0.z(), self.1.z())
    }

    fn z1(&self) -> usize {
        max(self.0.z(), self.1.z())
    }

    fn x0(&self) -> usize {
        min(self.0.x(), self.1.x())
    }

    fn x1(&self) -> usize {
        max(self.0.x(), self.1.x())
    }

    fn y0(&self) -> usize {
        min(self.0.y(), self.1.y())
    }

    fn y1(&self) -> usize {
        max(self.0.y(), self.1.y())
    }

    fn xy(&self) -> Vec<(usize, usize)> {
        (self.x0()..=self.x1())
            .cartesian_product(self.y0()..=self.y1())
            .collect()
    }

    fn drop(&mut self) {
        self.0 .2 -= 1;
        self.1 .2 -= 1;
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.z0(), self.x0(), self.y0()).cmp(&(other.z0(), other.y0(), other.z0()))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn name(i: usize) -> char {
    if i < 28 {
        (i + 65) as u8 as char
    } else {
        'X'
    }
}

fn process(input: &str) -> usize {
    let mut bricks = parse(input).expect("parses");
    bricks.sort();

    let mut supported_by = HashMap::new();
    for (n, i) in bricks.clone().into_iter().map(|b| b.i()).enumerate() {
        println!("{}/{}", n, bricks.len());
        while has_space_below(i, &bricks, &mut supported_by) {
            bricks.iter_mut().find(|b| b.i() == i).unwrap().drop();
        }
    }

    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, s) in supported_by.iter() {
        for b in s {
            supports.entry(*b).and_modify(|set|{ set.insert(*i);}).or_insert(HashSet::from([*i]));
        }
    }
    bricks
        .iter()
        .filter(|b| can_be_disintergrated(b, &supported_by, &supports))
        .count()
}

fn has_space_below(
    idx: usize,
    bricks: &Vec<Brick>,
    supported: &mut HashMap<usize, HashSet<usize>>,
) -> bool {
    let brick = bricks.iter().find(|b| b.i() == idx).unwrap();
    if brick.z0() == 1 {
        return false;
    }
    for xy in brick.xy() {
        for potential_support in bricks {
            if potential_support.i() == brick.i() || potential_support.z0() > brick.z0() {
                continue;
            }
            if potential_support.xy().contains(&xy) && brick.z0() - 1 == potential_support.z1()  {
                supported
                    .entry(idx)
                    .and_modify(|v| {
                        v.insert(potential_support.i());
                    })
                    .or_insert(HashSet::from([potential_support.i()]));
            }
        }
    }
    !supported.contains_key(&idx)
}

fn can_be_disintergrated(
    brick: &Brick,
    supported_by: &HashMap<usize, HashSet<usize>>,
    supports: &HashMap<usize, HashSet<usize>>,
) -> bool {
    if let Some(supports) = supports.get(&brick.i()) {
        let res = supports.iter().all(|s| {
            let s = supported_by
                .get(s)
                .expect("should have an entry if its supported");
            s.len() > 1
        });
        if res {
            println!(
                "{} can be disintegrated as it's supported bricks have other supports.",
                name(brick.i())
            );
        } else {
            println!(
                "{} cannot be disintegrated as it is the sole support for another block",
                name(brick.i())
            );
        }
        res
    } else {
        println!(
            "{} can be disintegrated as it does not support any other bricks.",
            name(brick.i())
        );
        true
    }
}

fn parse(input: &str) -> Result<Vec<Brick>, Box<dyn Error>> {
    Ok(input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let (p1, p2) = line.split_once('~')?;
            let p1: (usize, usize, usize) = p1
                .split(',')
                .filter_map(|n| n.parse().ok())
                .collect_tuple()?;
            let p2: (usize, usize, usize) = p2
                .split(',')
                .filter_map(|n| n.parse().ok())
                .collect_tuple()?;
            Some(Brick(p1.into(), p2.into(), i))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_volume() {
        let b = Brick(Vec3(0,0,1), Vec3(0,0,10),0);
        assert_eq!(b.xy().len(), 1);
    }

    #[test]
    fn test_volume2() {
        let b = Brick(Vec3(2,2,2), Vec3(2,2,2),0);
        assert_eq!(b.xy(), vec![(2,2)]);

        let c = Brick(Vec3(2,4,2), Vec3(2,2,2),0);
        assert_eq!(c.xy(), vec![(2,2),(2,3),(2,4)]);
    }

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 5);
    }
}

// A 0
// B 1
// C 2
// D 3
// E 4
// F 5
// G 6

//Brick A is the only brick supporting bricks B and C.
// Brick B is one of two bricks supporting brick D and brick E.
// Brick C is the other brick supporting brick D and brick E.
// Brick D supports brick F.
// Brick E also supports brick F.
// Brick F supports brick G.
// Brick G isn't supporting any bricks.
