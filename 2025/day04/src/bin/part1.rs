use common::{Direction, DirectionOps, Itertools, TooDee};

fn main() {
    let input: String = common::AocInput::fetch(2025, 4).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i32 {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let grid = TooDee::from_vec(
        width,
        height,
        input.replace('\n', "").trim().chars().collect(),
    );

    let mut accessible = 0;
    for point in (0..width).cartesian_product(0..height) {
        if grid[point] == '@'
            && Direction::ALL
                .into_iter()
                .filter(|dir| {
                    grid.move_point(&point, *dir)
                        .is_some_and(|cell| grid[cell] == '@')
                })
                .count()
                < 4
        {
            accessible += 1;
        }
    }

    accessible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 13);
    }
}
