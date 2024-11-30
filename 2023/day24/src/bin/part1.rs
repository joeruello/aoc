use std::ops::RangeInclusive;
use glam::{DVec2, DVec3, Vec3Swizzles};
use itertools::Itertools;


#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: DVec3,
    vel: DVec3,
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone) -> Option<DVec2> {
        let a = self.vel.y / self.vel.x;
        let c = self.pos.y - a * self.pos.x;

        let b = other.vel.y / other.vel.x;
        let d = other.pos.y - b * other.pos.x;

        if a == b && b != d {
            return None;
        }

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
        let px = (d - c) / (a - b);
        let py = a * ((d - c) / (a - b)) + c;

        Some((px, py).into()) 
    }

    fn is_moving_towards(&self, point: DVec2) -> bool {
        let vp = point - self.pos.xy();
        vp.dot(self.vel.xy()) > 0.0
    }
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input, 200000000000000.0..=400000000000000.0));
}

fn process(input: &str, bounds: RangeInclusive<f64>) -> usize {
    let stones = parse(input);

    let mut future_intersections = 0;
    for (a, b) in stones.into_iter().tuple_combinations() {
        if let Some(point) = a.intersects(&b) {
            if !(a.is_moving_towards(point) && b.is_moving_towards(point)) {
                continue;
            }
            if bounds.contains(&point.x) && bounds.contains(&point.y) {
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
                pos: (px, py, pz).into(),
                vel: (vx, vy, vz).into(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt"), 7.0..=27.0), 2);
    }
}
