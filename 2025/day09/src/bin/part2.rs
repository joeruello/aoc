use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap, VecDeque},
};

use common::{DirectionOps, Itertools, TooDee};

fn main() {
    let input: String = common::AocInput::fetch(2025, 9).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let xform = Compressed2D::new(input.lines().map(|l| {
        let (x, y) = l
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        (x, y)
    }));

    let (width, height) = xform.bounds();

    let mut grid = TooDee::<char>::init(width, height, '.');

    for ((x1, y1), (x2, y2)) in xform.cords.iter().circular_tuple_windows() {
        grid[(*x1, *y1)] = '#';
        if x1 == x2 {
            let ymin = *min(y1, y2);

            let ymax = *max(y1, y2);
            for y in (ymin + 1)..ymax {
                grid[(*x1, y)] = 'X'
            }
        } else if y1 == y2 {
            let xmin = *min(x2, x1);
            let xmax = *max(x1, x2);
            for x in (xmin + 1)..xmax {
                grid[(x, *y1)] = 'X'
            }
        }
    }

    flood_fill(&mut grid, (0, 0), '.', '_');

    let rects = xform.cords.iter().copied().tuple_combinations();
    //  .sorted_by_key(|(a, b)| {
    //      let a = xform.decompress(a).unwrap();
    //      let b = xform.decompress(b).unwrap();
    //      (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
    //  })
    //  .rev();

    let mut max_area = 0;

    for ((x1, y1), (x2, y2)) in rects {
        let a = xform.decompress(&(x1, y1)).unwrap();
        let b = xform.decompress(&(x2, y2)).unwrap();

        let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);

        if area < max_area {
            continue;
        }

        let xmin = min(x1, x2);
        let xmax = max(x1, x2);
        let ymin = min(y1, y2);
        let ymax = max(y1, y2);

        let a = (xmin, ymin);
        let b = (xmin, ymax);
        let c = (xmax, ymax);
        let d = (xmax, ymin);

        if [a, b, c, d].iter().any(|p| grid[*p] == '_') {
            continue;
        }

        // if [a, b, c, d].iter().filter(|p| grid[**p] != '#').count() < 3 {
        //    continue;
        // }

        if !((xmin..xmax)
            .cartesian_product(ymin..ymax)
            .all(|p| grid[p] != '_'))
        {
            continue;
        }

        // for (x, y) in (xmin..xmax).cartesian_product(ymin..ymax) {
        //     grid[(x, y)] = '█';
        // }
        //
        max_area = area;

        // for y in 0..height {
        //     for x in 0..width {
        //         print!("{}", grid[(x, y)])
        //     }
        //     println!();
        // }
    }

    max_area
}

struct Compressed2D {
    x_map: HashMap<usize, usize>,
    y_map: HashMap<usize, usize>,
    pub cords: Vec<(usize, usize)>,
}

fn flood_fill(grid: &mut TooDee<char>, start: (usize, usize), from: char, to: char) {
    let mut queue = VecDeque::new();
    queue.push_front(start);

    while let Some(p) = queue.pop_front() {
        if grid[p] != from {
            continue;
        }

        grid[p] = to;
        queue.extend(grid.neighbours(p));
    }
}

impl Compressed2D {
    fn new(cords: impl Iterator<Item = (usize, usize)> + Clone) -> Self {
        let mut xs = BTreeSet::new();
        let mut ys = BTreeSet::new();
        xs.insert(0);
        ys.insert(0);
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in cords.clone() {
            xs.insert(x);
            xs.insert(x + 1);
            ys.insert(y + 1);
            ys.insert(y);
            max_x = max(x + 1, max_x);
            max_y = max(y + 1, max_y);
        }

        xs.insert(max_x + 1);
        ys.insert(max_y + 1);

        let mut x_map = HashMap::new();
        let mut x_map_inv = HashMap::new();
        for (i, c) in xs.into_iter().enumerate() {
            x_map.insert(i, c);
            x_map_inv.insert(c, i);
        }

        let mut y_map = HashMap::new();
        let mut y_map_inv = HashMap::new();
        for (i, c) in ys.into_iter().enumerate() {
            y_map.insert(i, c);
            y_map_inv.insert(c, i);
        }

        let cords = cords
            .into_iter()
            .map(|(x, y)| {
                let x = *x_map.iter().find(|(_, x0)| **x0 == x).unwrap().0;
                let y = *y_map.iter().find(|(_, y0)| **y0 == y).unwrap().0;
                (x, y)
            })
            .collect_vec();

        Self {
            x_map,
            y_map,
            cords,
        }
    }

    fn decompress(&self, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
        let expanded_x = self.x_map.get(x)?;
        let expanded_y = self.y_map.get(y)?;
        Some((*expanded_x, *expanded_y))
    }

    fn bounds(&self) -> (usize, usize) {
        (
            *self.x_map.keys().max().unwrap(),
            *self.y_map.keys().max().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 25);
    }
}
