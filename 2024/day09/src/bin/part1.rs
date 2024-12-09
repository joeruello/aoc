use std::iter::repeat_n;

use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 9).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let expanded = expand(
        &input
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec(),
    );

    let compacted = compact(&expanded);

    compacted
        .into_iter()
        .enumerate()
        .map(|(f, i)| f * i.unwrap_or(0))
        .sum()
}

fn expand(map: &[usize]) -> Vec<Option<usize>> {
    let mut out = vec![];
    let mut file_id = 0;
    for (i, count) in map.iter().enumerate() {
        match i % 2 {
            0 => {
                out.extend(repeat_n(Some(file_id), *count).collect_vec());
                file_id += 1;
            }
            1 => out.extend(repeat_n(None, *count)),
            _ => unreachable!(),
        }
    }
    out
}

fn compact(input: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut reversed = input
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .collect_vec();

    let mut map = input.to_vec();

    for (i, _) in input.iter().enumerate().filter(|(_, c)| !c.is_some()) {
        let (ir, _) = reversed.pop().unwrap();
        if ir < i {
            break;
        }
        map.swap(ir, i);
    }
    map.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process("12345"), 60);

        assert_eq!(process(include_str!("./sample.txt")), 1928);
    }
}
