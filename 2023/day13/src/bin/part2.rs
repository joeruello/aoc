use itertools::Itertools;
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input: String = common::AocInput::fetch(2023, 13).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let patterns: Vec<_> = input.split("\n\n").collect();

    let patterns: Vec<_> = patterns
        .into_iter()
        .map(|pattern| {
            let width = pattern.chars().position(|c| c == '\n').expect("newline");
            let height = pattern.replace('\n', "").len() / width;
            TooDee::from_vec(
                width,
                height,
                pattern.replace('\n', "").trim().chars().collect(),
            )
        })
        .collect();

    patterns.into_iter().map(find_reflections).sum()
}

fn count_differences(a: &[char], b: &[char]) -> usize {
    let mut num_diffs = 0;
    let mut first_idx = None;
    for i in 0..a.len() {
        if a[i] != b[i] {
            if first_idx.is_none() {
                first_idx = Some(i);
            }
            num_diffs += 1;
        }
    }
    num_diffs
}

fn find_reflections(pattern: TooDee<char>) -> usize {
    if let Some(row) = (0..pattern.num_rows())
        .tuple_windows()
        .find_map(|(ia, ib)| {
            let mut fixed_smudge = false;
            for i in 0.. {
                if i > ia || ib + i >= pattern.num_rows() {
                    break;
                }
                match count_differences(&pattern[ia - i], &pattern[ib + i]) {
                    0 => continue,
                    1 => {
                        if fixed_smudge {
                            return None;
                        }
                        fixed_smudge = true
                    }
                    _ => return None,
                }
            }

            fixed_smudge.then_some(ia)
        })
    {
        (row + 1) * 100
    } else if let Some(col) = (0..pattern.num_cols())
        .tuple_windows()
        .find_map(|(ia, ib)| {
            let mut fixed_smudge = false;
            for i in 0.. {
                if i > ia || ib + i >= pattern.num_cols() {
                    break;
                }
                let col_a = pattern.col(ia - i).cloned().collect_vec();
                let col_b = pattern.col(ib + i).cloned().collect_vec();
                match count_differences(&col_a, &col_b) {
                    0 => continue,
                    1 => {
                        if fixed_smudge {
                            return None;
                        }
                        fixed_smudge = true
                    }
                    _ => return None,
                }
            }
            fixed_smudge.then_some(ia)
        })
    {
        col + 1
    } else {
        panic!("no matches")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 400);
    }
}
