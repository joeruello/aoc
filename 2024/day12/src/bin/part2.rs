use std::collections::{HashMap, HashSet, VecDeque};

use common::{Direction, DirectionOps, Itertools};
use toodee::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2024, 12).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.find("\n").unwrap();
    let height = input.lines().count();
    let grid = TooDee::from_vec(
        width,
        height,
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    );

    let mut sum = 0;

    let mut visited = HashSet::new();
    let mut regions = VecDeque::from([(0, 0)]);

    while let Some(region) = regions.pop_front() {
        let mut area = 0;
        let mut current = VecDeque::from([region]);
        let mut points = HashSet::new();
        let mut perim = HashMap::new();
        while let Some(p) = current.pop_front() {
            if visited.contains(&p) {
                continue;
            }
            area += 1;
            points.insert(p);

            for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
                if let Some(n) = grid.move_point(&p, dir.xy()) {
                    if grid[n] == grid[p] {
                        current.push_back(n);
                    } else {
                        perim.entry(p).or_insert(vec![]).push(dir);
                        if !visited.contains(&n) {
                            regions.push_back(n);
                        }
                    }
                } else {
                    perim.entry(p).or_insert(vec![]).push(dir);
                }
            }
            visited.insert(p);
        }
        if grid[region] == '.' {
            continue;
        }

        let mut corners = HashSet::new();
        let mut inner_corners = HashSet::new();

        for (p, dirs) in perim.iter() {
            let mut dirs = dirs.clone();
            dirs.sort();
            if dirs.len() == 4 {
                corners.insert((p, Direction::NW));
                corners.insert((p, Direction::NE));
                corners.insert((p, Direction::SW));
                corners.insert((p, Direction::SE));
                continue;
            }

            for (a, b) in dirs.iter().circular_tuple_windows() {
                if a == b {
                    continue;
                }
                match &(a, b) {
                    &(Direction::N, Direction::S)
                    | &(Direction::E, Direction::W)
                    | &(Direction::S, Direction::N)
                    | &(Direction::W, Direction::E) => {
                        continue;
                    }
                    _ => {
                        corners.insert((p, a.bisect(b).expect("")));
                    }
                }
            }
        }

        for (p, dirs) in perim.iter() {
            if dirs.len() == 4 {
                continue;
            }
            for dir in dirs {
                let conf = HashMap::from([
                    (
                        Direction::N,
                        [
                            (Direction::NE, Direction::E, Direction::SE),
                            (Direction::NW, Direction::W, Direction::SW),
                        ],
                    ),
                    (
                        Direction::S,
                        [
                            (Direction::SE, Direction::E, Direction::NE),
                            (Direction::SW, Direction::W, Direction::NW),
                        ],
                    ),
                    (
                        Direction::E,
                        [
                            (Direction::SE, Direction::S, Direction::SW),
                            (Direction::NE, Direction::N, Direction::NW),
                        ],
                    ),
                    (
                        Direction::W,
                        [
                            (Direction::SW, Direction::S, Direction::SE),
                            (Direction::NW, Direction::N, Direction::NE),
                        ],
                    ),
                ]);

                for (search, next, corner) in conf.get(dir).unwrap() {
                    if grid
                        .move_point(p, search.xy())
                        .is_some_and(|np| perim.contains_key(&np))
                        && grid
                            .move_point(p, next.xy())
                            .is_some_and(|np| grid[np] == grid[*p])
                    {
                        let point = grid.move_point(p, dir.xy()).unwrap();
                        inner_corners.insert((point, *corner));
                    }
                }
            }
        }

        sum += area * (corners.len() + inner_corners.len());
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(process(include_str!("./case1.txt")), 60);
        assert_eq!(process(include_str!("./case2.txt")), 18 * 4);
        assert_eq!(process(include_str!("./case3.txt")), 18 * 2);
        assert_eq!(process(include_str!("./case4.txt")), 60 * 34);
        assert_eq!(process(include_str!("./case5.txt")), 8 * 8);
        assert_eq!(process(include_str!("./case6.txt")), 236);
        assert_eq!(process(include_str!("./case7.txt")), 368);
        assert_eq!(process(include_str!("./case9.txt")), 28 * 12);
    }

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 1206);
    }
}
