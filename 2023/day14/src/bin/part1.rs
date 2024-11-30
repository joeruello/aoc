use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let mut grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    tilt_north(&mut grid);


    let mut sum = 0;
    for (i, row) in grid.rows().enumerate() {
        let count = row.iter().filter(|c| **c == '0').count();
        sum += count * (grid.num_rows() - i)
    }

    sum
}

fn tilt_north(grid: &mut TooDee<char>) {
    for x in 0..grid.num_cols() {
        let col: Vec<char> = grid.col(x).cloned().collect();
        let new_col = collapse(&col);
        for (i, c) in grid.col_mut(x).enumerate() {
            *c = new_col[i];
        }
    }
}

fn collapse(pattern: &[char]) -> Vec<char> {
    let chunks: Vec<_> = pattern.split_inclusive(|c| *c == '#').collect();
    chunks
        .into_iter()
        .flat_map(|chunk| {
            println!("{chunk:?} {}", chunk.len());
            let count = chunk.iter().filter(|c| **c == 'O').count();
            println!("{count} rocks");
            let mut ret = vec![];
            ret.append(&mut ['0'].repeat(count));
            ret.append(&mut ['.'].repeat(chunk.len() - count));
            if *chunk.last().unwrap() == '#' {
                ret[chunk.len() - 1] = '#';
            }
            ret
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 136);
    }
}
