use std::collections::HashSet;

use toodee::{TooDee, TooDeeOps};

type Grid = TooDee<char>;
type Cordinate = (isize, isize);

fn neighbours((x, y): Cordinate, grid: &Grid, bounds: Cordinate) -> Vec<Cordinate> {
    let n = Some((x, y - 1));
    let s = Some((x, y + 1));
    let e = Some((x + 1, y));
    let w = Some((x - 1, y));

    vec![n, s, e, w]
        .into_iter()
        .filter_map(|cord| cord.filter(|(x, y)| grid[wrap_cordinate((*x, *y), bounds)] != '#'))
        .collect()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));

}

fn wrap_cordinate((x, y): Cordinate, (w, h): Cordinate) -> (usize, usize) {
    (
        (x.rem_euclid(w)).unsigned_abs(),
        (y.rem_euclid(h)).unsigned_abs(),
    )
}

fn process(input: &str) -> isize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    // Because the row/col of the S is all blank, the number of tiles visited increases 
    // quadtrically every *width* (or height because its square) tiles
    // We find the sequence, and then solve for the target number which isa multiple of 
    // the grid size - half the grid size (because the first grid we start in the middle)
    // (This is a crafted input and not a general fact)

    let target = 26_501_365;
    let half_grid_len = width/2;
    let num_grids_walked = ((target - half_grid_len) / width)  as isize;

    println!("Number of grids walked to target({target}): {num_grids_walked}");

    let walk_to_end_once = walk(&grid, half_grid_len) as isize; // 65
    println!("Walking to edge of one grid ({} tiles): {walk_to_end_once}", half_grid_len);

    let plus_one_grid = walk(&grid, width + width/2) as isize;

    println!("Walking 1 grid({} tiles): {plus_one_grid}", width + half_grid_len);

    let plus_two_grid = walk(&grid, width *2 + width/2);
    println!("Walking 2 grid({} tiles: {plus_two_grid}", width*2 + half_grid_len);

    let plus_three_grid = walk(&grid, width *3 + width/2);
    println!("Walking 3 grid({} tiles: {plus_three_grid}", width*3 + half_grid_len);

    let first_difference_0 = (plus_one_grid as isize - walk_to_end_once) as isize;
    let first_difference_1 = (plus_two_grid as isize - plus_one_grid) as isize;
    let first_difference_2  = (plus_three_grid - plus_two_grid) as isize;

    let second_difference_0 = first_difference_1 - first_difference_0; 
    let second_difference_1 = first_difference_2 - first_difference_1;

    // Confirm that the increase on full grid walks is infact a quadraric sequence
    assert!(second_difference_0 == second_difference_1);

    // https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
    // Get parameters for quadatric equation: ax^2 + bx + c
    // 2a = second_difference
    let a = second_difference_0 / 2;
    // 3a + b = first_difference
    let b = first_difference_1 - (3 * a);
    //  a + b +c = y0
    let c = plus_one_grid - a - b;

    println!("Solving {a}x^2 + {b}x + {c} for x = {num_grids_walked}");
    (a * num_grids_walked * num_grids_walked) + (b * num_grids_walked) + c
}

fn walk(grid: &Grid, num_steps: usize) -> usize {
    let bounds = (grid.num_cols() as isize, grid.num_rows() as isize);
    let mut start = (isize::MAX, isize::MAX);

    'outer: for y in 0..grid.num_rows() {
        for x in 0..grid.num_cols() {
            if grid[(x, y)] == 'S' {
                start = (x as isize, y as isize);
                break 'outer;
            }
        }
    }

    let mut next: HashSet<Cordinate> = neighbours(start, grid, bounds).into_iter().collect();
    for _ in 1..num_steps {
        let mut next_set = vec![];
        for cord in next.into_iter() {
            next_set.append(&mut neighbours(cord, grid, bounds));
        }
        next = next_set.into_iter().collect();

    }
    next.len()
}
