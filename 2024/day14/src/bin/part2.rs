use common::Itertools;
use glam::{ivec2, IVec2};

fn main() {
    let input: String = common::AocInput::fetch(2024, 14).unwrap().into();
    println!("Output: {}", process(&input, 101, 103));
}

fn process(input: &str, width: i32, height: i32) -> i32 {
    let grid = ivec2(width, height);
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

    for n in 0.. {
        if let Some(steps) = simulate(&robots, n, grid) {
            return steps;
        }
    }
    unreachable!()
}

fn simulate(robots: &[(IVec2, IVec2)], steps: i32, grid: IVec2) -> Option<i32> {
    let counts = robots
        .iter()
        .map(|(p, v)| (p + (v * steps)).rem_euclid(grid))
        .counts();

    for y in 0..grid.y {
        let mut row = 0;
        for x in 0..grid.x {
            if counts.contains_key(&ivec2(x, y)) {
                row += 1;
                // if we find 10 robots in a row is probally the tree
                if row >= 10 {
                    return Some(steps);
                }
            } else {
                row = 0;
            }
        }
    }
    None
}
