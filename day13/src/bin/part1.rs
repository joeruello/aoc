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
            TooDee::from_vec(width, height, pattern.replace('\n', "").trim().chars().collect())
        })
        .collect();

    patterns.into_iter().map(find_reflections).sum()
}

fn find_reflections(pattern: TooDee<char>) -> usize {
    let candidate_rows = pattern
        .rows()
        .enumerate()
        .tuple_windows()
        .filter(|((ia, a), (ib, b))| {
            if a != b {
                return false;
            }

            for i in 1.. {
                if i > *ia || *ib + i >= pattern.num_rows() {
                    break;
                }
                if pattern[ia - i] != pattern[ib + i] {
                    return false;
                }
            }

            true
        })
        .collect_vec();

    if let Some(((ia, _), _)) = candidate_rows.first() {
        println!("Found row {}", ia + 1);
        return (*ia + 1) * 100;
    }

    let candidate_cols = (0..pattern.num_cols()).tuple_windows().filter(|(ia, ib)| {
        let a = pattern.col(*ia).collect_vec();
        let b = pattern.col(*ib).collect_vec();

        if a != b {
            return false;
        }
        println!("Candiate {a:?} {b:?}");


        for i in 1.. {
            if i > *ia || *ib + i >= pattern.num_cols() {
                break;
            }
            if pattern.col(ia.saturating_sub(i)).collect_vec() != pattern.col(ib + i).collect_vec() {
                return false;
            }
        }

        true
    }).collect_vec();

    if let Some((ia, _)) = candidate_cols.first() {
        println!("Found col {}", ia + 1);
        return *ia + 1
    }

    panic!("no matches")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 405);
    }
}
