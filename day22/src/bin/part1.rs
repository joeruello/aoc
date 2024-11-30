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
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let mut bricks = parse(input).expect("parses");
    bricks.sort();

    let mut supported_by = HashMap::new();
    for (_, i) in bricks.clone().into_iter().map(|b| b.i()).enumerate() {
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
    bricks: &[Brick],
    supported: &mut HashMap<usize, HashSet<usize>>,
) -> bool {
    let brick = bricks.iter().find(|b| b.i() == idx).unwrap();
    if brick.z0() == 1 {
        return false;
    }
    let candiate_bricks = bricks.iter().filter(|b| brick.z0() -1 == b.z1()).collect_vec();
    for xy in brick.xy() {
        for potential_support in candiate_bricks.iter() {
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
        res
    } else {
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
