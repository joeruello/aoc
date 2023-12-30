use std::ops::{RangeInclusive, SubAssign, AddAssign};

use itertools::Itertools;
use num::{BigRational, FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy)]
struct Vec3(f64, f64, f64);


impl Vec3 {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }

    fn sum_parts(&self) -> f64 {
        self.0 + self.1 + self.2
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.vel.x() == 0.0 || other.vel.x() == 0.0 {
            return Some((f64::NAN, f64::NAN));
        }

        let a = BigRational::from_f64(self.vel.y())? / BigRational::from_f64(self.vel.x())?;
        let c = BigRational::from_f64(self.pos.y())? - a.clone() * BigRational::from_f64(self.pos.x())?;

        let b = BigRational::from_f64(other.vel.y())? / BigRational::from_f64(other.vel.x())?;
        let d = BigRational::from_f64(other.pos.y())? - (b.clone()  * BigRational::from_f64(other.pos.x())?);

        if a == b && b != d {
            return Some((f64::NAN, f64::NAN));
        }

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_line_equations
        let px = (d.clone() - c.clone()) / (a.clone() - b.clone());
        let py = a.clone() * ((d - c.clone()) / (a - b)) + c;

        Some((px.to_f64()?, py.to_f64()?))
    }

    fn is_moving_towards(&self, point: (f64, f64)) -> bool {
        // vector towards then point
        let vx = point.0 - self.pos.x();
        let vy = point.1 - self.pos.y();

        // dot product w/ velocity
        vx * self.vel.x() + vy * self.vel.y() > 0.0
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
                pos: Vec3(px, py, pz),
                vel: Vec3(vx, vy, vz),
            }
        })
        .collect()
}

fn process(input: &str, search_space: RangeInclusive<isize>) -> f64 {
    let stones = parse(input);
    for vel in search_space.clone().cartesian_product(search_space.clone()) {
        if let Some(collision_point) = all_intersect(vel, &stones) {
            for z in search_space.clone() {
                if let Some(origin) = all_intersect_with_z(
                    Vec3(vel.0 as f64, vel.1 as f64, z as f64),
                    collision_point,
                    &stones,
                ) {
                    return origin.sum_parts()
                }
            }
        }
    }
    panic!("no solution found")
}

fn all_intersect((x, y): (isize, isize), stones: &[Hailstone]) -> Option<(f64, f64)> {
    let origin_vel = Vec3(x as f64, y as f64, 0.0);
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
            if point.0.fract() != 0.0 || point.1.fract() != 0.0 {
                return None;
            }
            collision_point.get_or_insert(point);
        } else {
            return None;
        }
    }

    collision_point
}


fn all_intersect_with_z(v: Vec3, collision: (f64, f64), stones: &[Hailstone]) -> Option<Vec3> {
    let mut z = None;
    let mut point = None;
    for (a0, b0) in stones.iter().tuple_combinations() {
        let mut a = *a0;
        let mut b = *b0;

        a.vel -= v;
        b.vel -= v;

        let ta = (a.pos.x() - collision.0) / -(a.vel.x());
        let tb = (b.pos.x() - collision.0) / -(b.vel.x());
  
        let za = a.pos.z() + a.vel.z() * ta;
        let zb = b.pos.z() + b.vel.z() * tb;

        if za != zb {
            return None;
        }

        if z.is_none() {
            z = Some(za);
            let xa = a.pos.x() + a.vel.x() * ta;
            let ya = a.pos.y() + a.vel.y() * ta;

            let xb = b.pos.x() + b.vel.x() * tb;
            let yb = b.pos.y() + b.vel.y() * tb;

            let vx = (xb - xa) / (tb - ta);
            let vy = (yb - ya) / (tb - ta);
            let vz = (zb - za) / (tb - ta);

            let inital_x = collision.0 - ta * vx;
            let inital_y = collision.1 - ta * vy;
            let inital_z = za - ta * vz;

            point = Some(Vec3(inital_x, inital_y, inital_z));

        } else if za != z.unwrap() {
            return None;
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
