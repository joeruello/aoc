use std::collections::{HashMap, VecDeque};
use toodee::{TooDee, TooDeeOps};
type Cordinate = (usize, usize);
type Grid = TooDee<Tile>;

#[derive(Debug, PartialEq)]
enum Tile {
    Path,
    Forest,
}

#[derive(Debug)]
struct State {
    coords: Cordinate,
    distance: usize,
    visited: Vec<Cordinate>,
}

fn neighbours((x, y): Cordinate, grid: &Grid) -> Vec<(Cordinate, usize)> {
    let n = (y > 0).then_some((x, y.saturating_sub(1)));
    let s = (y < grid.num_rows() - 1).then_some((x, y + 1));
    let w = (x > 0).then_some((x.saturating_sub(1), y));
    let e = (x < grid.num_cols() - 1).then_some((x + 1, y));

    vec![n, s, e, w]
        .into_iter()
        .filter_map(|f| {
            if let Some(cords) = f {
                match grid[cords] {
                    Tile::Path => Some((cords, 1)),
                    Tile::Forest => None,
                }
            } else {
                None
            }
        })
        .collect()
}

fn parse(input: &str) -> Grid {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    TooDee::from_vec(
        width,
        height,
        input
            .replace('\n', "")
            .trim()
            .chars()
            .map(|c| match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '>' => Tile::Path,
                '<' => Tile::Path,
                '^' => Tile::Path,
                'v' => Tile::Path,
                _ => panic!("unknown tile {c}"),
            })
            .collect(),
    )
}

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn find_start(grid: &Grid) -> Cordinate {
    let first_row = grid.rows().next().unwrap();
    let start_x = first_row.iter().position(|t| *t == Tile::Path).unwrap();

    (start_x, 0)
}

fn find_end(grid: &Grid) -> Cordinate {
    let last_row = grid.rows().last().unwrap();
    let end_x = last_row.iter().position(|t| *t == Tile::Path).unwrap();

    (end_x, grid.num_rows() - 1)
}

fn process(input: &str) -> usize {
    let grid = parse(input);
    let start = find_start(&grid);
    let end = find_end(&grid);

    let mut edges = HashMap::new();
    for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            if let Tile::Path = grid[(x, y)] {
                edges.insert((x, y), neighbours((x, y), &grid));
            }
        }
    }

    collapse_edges(&mut edges);
    find_longest_path(&edges, start, end)
}

fn collapse_edges(edges: &mut HashMap<Cordinate, Vec<(Cordinate, usize)>>) {
    let nodes: Vec<_> = edges.keys().cloned().collect();

    for node in nodes {
        if let Some(selected_edges) = edges.get(&node).cloned() {
            if selected_edges.len() == 2 {
                let (n1, d1) = selected_edges.first().unwrap();
                let (n2, d2) = selected_edges.last().unwrap();

                edges.entry(*n1).and_modify(|f| {
                    let (idx, (_, dist)) =
                        f.iter().enumerate().find(|(_, (c, _))| *c == node).unwrap();
                    f[idx] = (*n2, dist + d2);
                });

                edges.entry(*n2).and_modify(|f| {
                    let (idx, (_, dist)) =
                        f.iter().enumerate().find(|(_, (c, _))| *c == node).unwrap();
                    f[idx] = (*n1, dist + d1);
                });

                edges.remove(&node);
            }
        }
    }
}

fn find_longest_path(
    edges: &HashMap<Cordinate, Vec<(Cordinate, usize)>>,
    start: Cordinate,
    dest: Cordinate,
) -> usize {
    let mut queue = VecDeque::<State>::new();

    let mut iters = 0;
    queue.push_back(State {
        coords: start,
        distance: 0,
        visited: vec![],
    });

    let mut goal_distances = vec![];

    while let Some(State {
        coords,
        distance: cost,
        mut visited,
    }) = queue.pop_front()
    {
        iters += 1;
        if coords == dest {
            goal_distances.push(cost);
            continue;
        }
        visited.push(coords);
        for (neighbour_coordinate, distance) in edges.get(&coords).unwrap() {
            if visited.contains(neighbour_coordinate) {
                continue;
            }
            queue.push_back(State {
                distance: cost + distance,
                coords: *neighbour_coordinate,
                visited: visited.clone(),
            });
        }
        if iters % 500000 == 0 {
            println!("{iters} - Checking {coords:?}@{cost}");
            println!("Current max: {:?}", goal_distances.iter().max());
            println!("Solutions: {:?}", goal_distances.len());
        }
    }

    goal_distances.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 154);
    }
}
