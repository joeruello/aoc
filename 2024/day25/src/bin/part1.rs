use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 25).unwrap().into();
    println!("Output: {}", process(&input));
}

const LINE_LEN: usize = 5;
const SCHEM_HEIGHT: usize = 6;
fn process(input: &str) -> usize {
    let schematics = input.split("\n\n");
    let mut locks = vec![];
    let mut keys = vec![];

    for schematic in schematics {
        let is_lock = schematic[0..LINE_LEN] == *"#####";
        let pattern = schematic.lines().skip(1).take(SCHEM_HEIGHT - 1).fold(
            [0, 0, 0, 0, 0],
            |mut arr, line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        arr[i] += 1;
                    }
                });
                arr
            },
        );
        if is_lock {
            locks.push(pattern);
        } else {
            keys.push(pattern);
        }
    }

    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip(key).all(|(a, b)| a + b <= 5))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_secrets() {
        assert_eq!(process(include_str!("./sample.txt")), 3)
    }
}
