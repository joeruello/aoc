use common::Itertools;
use glam::{ivec2, IVec2};

fn main() {
    let input: String = common::AocInput::fetch(2024, 14).unwrap().into();
    println!("Output: {}", process(&input, 101, 103));
}

fn process(input: &str, width: i32, height: i32) -> usize {
    let grid = ivec2(width, height);
    let steps = 100;
    let robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let px = px.parse::<i32>().unwrap();
            let py = py.parse::<i32>().unwrap();
            let (_, v) = v.split_once("=").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            let vx = vx.parse::<i32>().unwrap();
            let vy = vy.parse::<i32>().unwrap();
            (ivec2(px, py), ivec2(vx, vy))
        })
        .collect_vec();

    let simed = robots
        .into_iter()
        .map(|(p, v)| (p + (v * steps)).rem_euclid(grid));

    let counts = simed.clone().counts();
    for y in 0..height {
        for x in 0..width {
            if let Some(count) = counts.get(&ivec2(x, y)) {
                print!("{count}")
            } else {
                print!(".")
            }
        }
        println!()
    }

    simed
        .into_iter()
        .map(|p| quadrant(&p, &grid))
        .filter(|q| *q > 0)
        .counts()
        .values()
        .product()
}

fn quadrant(p: &IVec2, grid: &IVec2) -> usize {
    let w_half = grid.x / 2;
    let h_half = grid.y / 2;

    if p.x < w_half && p.y < h_half {
        return 1;
    } else if p.x > w_half && p.y < h_half {
        return 2;
    } else if p.x < w_half && p.y > h_half {
        return 3;
    } else if p.x > w_half && p.y > h_half {
        return 4;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt"), 11, 7), 12);
    }
}
