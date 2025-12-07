use std::collections::BTreeMap;

fn main() {
    let input: String = common::AocInput::fetch(2025, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let mut transposed: BTreeMap<usize, Vec<char>> = BTreeMap::new();

    input.lines().for_each(|l| {
        l.char_indices().for_each(|(x, c)| {
            let e = transposed.entry(x).or_default();
            e.push(c);
        });
    });

    let mut problems: Vec<(Vec<usize>, char)> = vec![(vec![], '+')];
    for col in transposed.values() {
        if col.iter().all(|c| c.is_whitespace()) {
            problems.push((vec![], '+'));
            continue;
        }

        let mut digit_found = false;
        let num = &col[..col.len()]
            .iter()
            .map(|c| match c.is_ascii_digit() {
                true => {
                    digit_found = true;
                    c
                }
                false => {
                    if digit_found {
                        c
                    } else {
                        &'0'
                    }
                }
            })
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let last = *col.last().unwrap();
        let current = problems.last_mut().unwrap();
        match last {
            '*' | '+' => {
                current.1 = last;
                current.0.push(*num);
            }
            _ => {
                current.0.push(*num);
            }
        };
    }
    problems
        .into_iter()
        .map(|(nums, op)| match op {
            '+' => nums.into_iter().sum::<usize>(),
            '*' => nums.into_iter().product(),
            _ => panic!("ohno"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 3263827);
    }
}
