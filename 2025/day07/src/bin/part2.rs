use common::TooDee;

fn main() {
    let input: String = common::AocInput::fetch(2025, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').expect("newline");
    let height = input.replace('\n', "").len() / width;
    let mut grid = TooDee::from_vec(
        width,
        height,
        input
            .replace('\n', "")
            .trim()
            .chars()
            .map(|c| (c, 0usize))
            .collect(),
    );

    for y in 0..height {
        for x in 0..width {
            let (char, _) = grid[(x, y)];

            if char == 'S' {
                grid[(x, y)].1 = 1;
            }

            if char == '.' && y > 0 {
                grid[(x, y)].1 += grid[(x, y - 1)].1;
            }
            if char == '^' && y > 0 {
                let from = grid[(x, y - 1)].1;
                for dx in [-1isize, 1isize] {
                    let nx = x as isize + dx;
                    if (0..width as isize).contains(&nx) {
                        let nx = nx as usize;
                        grid[(nx, y)].1 += from;
                    }
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            print!("{:?}", grid[(x, y)]);
        }
        println!();
    }

    (0..width).map(|x| grid[(x, height - 1)].1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 40);
    }
}
