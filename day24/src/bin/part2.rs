use glam::{DVec2, DVec3, Vec3Swizzles};
use itertools::Itertools;
use num::{BigRational, FromPrimitive, ToPrimitive};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: DVec3,
    vel: DVec3,
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone) -> Option<DVec2> {
        if self.vel.xy() == DVec2::ZERO {
            return Some(DVec2::NAN);
        }

        let a = BigRational::from_f64(self.vel.y)? / BigRational::from_f64(self.vel.x)?;
        let c = BigRational::from_f64(self.pos.y)? - a.clone() * BigRational::from_f64(self.pos.x)?;

        let b = BigRational::from_f64(other.vel.y)? / BigRational::from_f64(other.vel.x)?;
        let d =
            BigRational::from_f64(other.pos.y)? - (b.clone() * BigRational::from_f64(other.pos.x)?);

        if a == b && b != d {
            return Some(DVec2::NAN);
        }

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
        let px = (d.clone() - c.clone()) / (a.clone() - b.clone());
        let py = a.clone() * ((d - c.clone()) / (a - b)) + c;

        Some((px.to_f64()?, py.to_f64()?).into())
    }

    fn is_moving_towards(&self, point: DVec2) -> bool {
        let vp = point - self.pos.xy();
        vp.dot(self.vel.xy()) > 0.0
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input, -500..=500));
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

fn process(input: &str, search_space: RangeInclusive<i32>) -> f64 {
    let stones = parse(input);
    for (x, y) in search_space.clone().cartesian_product(search_space.clone()) {
        let vel = DVec2::new(x as f64, y as f64);
        if let Some(collision_point) = all_intersect(vel, &stones) {
            for z in search_space.clone() {
                if let Some(origin) =
                    all_intersect_with_z(vel.extend(z as f64), collision_point, &stones)
                {
                    return origin.x + origin.y + origin.z
                }
            }
        }
    }
    panic!("no solution found")
}

fn all_intersect(point: DVec2, stones: &[Hailstone]) -> Option<DVec2> {
    let origin_vel = point.extend(0.0);
    let mut collision_point = None;
    for (a0, b0) in stones.iter().tuple_combinations() {
        let mut a = *a0;
        let mut b = *b0;

        a.vel -= origin_vel;
        b.vel -= origin_vel;

        if let Some(point) = a.intersects(&b) {
            if !(a.is_moving_towards(point) && b.is_moving_towards(point)) {
                continue;
            }
            if point.fract() != DVec2::ZERO {
                return None;
            }
            collision_point.get_or_insert(point);
        } else {
            return None;
        }
    }

    collision_point
}

fn all_intersect_with_z(v: DVec3, collision: DVec2, stones: &[Hailstone]) -> Option<DVec3> {
    let mut point: Option<DVec3> = None;
    for (a0, b0) in stones.iter().tuple_combinations() {
        let mut a = *a0;
        let mut b = *b0;

        a.vel -= v;
        b.vel -= v;

        let ta = (a.pos.x - collision.x) / -(a.vel.x);
        let tb = (b.pos.x - collision.x) / -(b.vel.x);

        let at = a.pos + a.vel * ta;
        let bt = b.pos + b.vel * tb;

        if at.z != bt.z || point.is_some_and(|p| p.z != at.z) {
            return None;
        } else if point.is_none() {
            let v = (bt - at) / (tb -ta);
            let i = collision.extend(at.z) - ta * v;
            point = Some(i);
        }
    }

    point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt"), -5..=5), 47.0);
    }
}
