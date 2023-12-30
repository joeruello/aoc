use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Vec3(f64, f64, f64);
impl Vec3 {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let a = self.vel.y() / self.vel.x();
        let c = self.pos.y() - a * self.pos.x();

        let b = other.vel.y() / other.vel.x();
        let d = other.pos.y() - b * other.pos.x();

        if a == b && b != d {
            return None;
        }

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
        let px = (d - c) / (a - b);
        let py = a * ((d - c) / (a - b)) + c;

        Some((px, py))
    }

    fn is_moving_towards(&self, point: (f64,f64)) -> bool {
        // vector towards then point
        let vx = point.0 - self.pos.x();
        let vy = point.1 - self.pos.y();

        // dot product w/ velocity
        vx * self.vel.x() + vy * self.vel.y() > 0.0
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, 200000000000000.0..=400000000000000.0));
}

fn process(input: &str, bounds: RangeInclusive<f64>) -> usize {
    let stones = parse(input);

    let mut future_intersections = 0;
    for (a, b) in stones.into_iter().tuple_combinations() {
        if let Some(point) = a.intersects(&b) {
            if !(a.is_moving_towards(point) && b.is_moving_towards(point)) {
                continue;
            }
            if bounds.contains(&point.0) && bounds.contains(&point.1) {
                future_intersections +=1
            }
        }
    }

    future_intersections
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let (px, py, pz) = p
                .split(", ")
                .map(|p| p.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (vx, vy, vz) = v
                .split(", ")
                .map(|p| p.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            Hailstone {
                pos: Vec3(px, py, pz),
                vel: Vec3(vx, vy, vz),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt"), 7.0..=27.0), 405);
    }
}
