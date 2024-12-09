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
        .flat_map(|(fid, c)| repeat_n(fid, c))
        .enumerate()
        .map(|(f, i)| f * i.unwrap_or(0))
        .sum()
}

fn expand(map: &[usize]) -> Vec<(Option<usize>, usize)> {
    let mut out = vec![];
    let mut file_id = 0;
    map.iter().copied().enumerate().for_each(|(i, count)| {
        if i % 2 == 0 {
            out.push((Some(file_id), count));
            file_id += 1;
        } else if count > 0 {
            out.push((None, count))
        }
    });
    out
}

fn compact(input: &[(Option<usize>, usize)]) -> Vec<(Option<usize>, usize)> {
    let mut out = input.to_vec();
    for (file_id, file_len) in input.iter().rev().filter(|(fid, _)| fid.is_some()) {
        let slot = out
            .iter()
            .find_position(|(fid, flen)| fid.is_none() && flen >= file_len);

        if let Some((slot_idx, (_, gap_len))) = slot {
            let (file_idx, _) = out.iter().find_position(|(ni, _)| ni == file_id).unwrap();
            if file_idx < slot_idx {
                continue;
            }
            let diff = *gap_len - *file_len;
            out[slot_idx] = out[file_idx];
            out[file_idx] = (None, *file_len);

            if diff > 0 {
                out.insert(slot_idx + 1, (None, diff));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 2858);
    }
}
