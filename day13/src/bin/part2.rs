use itertools::Itertools;
use toodee::{TooDee, TooDeeOps};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
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
    assert!(a.len() == b.len());
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

    if num_diffs == 1 {
        println!(
            "Fixing smudge {a:?} {b:?}. Changed idx: {}",
            first_idx.unwrap()
        );
    }
    num_diffs
}

fn find_reflections(pattern: TooDee<char>) -> usize {
    println!("Finding reflections....");
    let candidate_rows = pattern
        .rows()
        .enumerate()
        .tuple_windows()
        .find(|((ia, a), (ib, b))| {
            let mut fixed_smudge = false;

            if count_differences(a, b) == 1 {
                fixed_smudge = true
            } else if a != b {
                return false;
            }

            for i in 1.. {
                if i > *ia || *ib + i >= pattern.num_rows() {
                    break;
                } else if pattern[ia - i] != pattern[ib + i] {
                    if fixed_smudge {
                        return false;
                    } else if count_differences(&pattern[ia - i], &pattern[ib + i]) == 1 && !fixed_smudge {
                        fixed_smudge = true;
                    } else {
                        return false;
                    }
                }
            }

            fixed_smudge
        });
    if let Some(((ia, _), _)) = candidate_rows {
        println!("  Found row {}", ia + 1);
        return (ia + 1) * 100;
    }

    let candidate_cols = (0..pattern.num_cols()).tuple_windows().find(|(ia, ib)| {
        let a = pattern.col(*ia).map(|c| c.to_owned()).collect_vec();
        let b = pattern.col(*ib).map(|c| c.to_owned()).collect_vec();
        let mut fixed_smudge = false;

        if count_differences(&a, &b) == 1 {
            fixed_smudge = true
        } else if a != b {
            return false;
        }

        for i in 1.. {
            if i > *ia || *ib + i >= pattern.num_cols() {
                break;
            }
            let col_a = pattern
                .col(ia.saturating_sub(i))
                .map(|c| c.to_owned())
                .collect_vec();
            let col_b = pattern.col(ib + i).map(|c| c.to_owned()).collect_vec();
            if col_a != col_b {
                if fixed_smudge {
                    return false;
                } else if count_differences(&col_a, &col_b) == 1 && !fixed_smudge {
                    fixed_smudge = true;
                } else {
                    return false;
                }
            }
        }
        fixed_smudge
    });

    if let Some((ia, _)) = candidate_cols {
        println!("Found col {}", ia + 1);
        return ia + 1;
    }

    panic!("no matches")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 400);
    }
}
