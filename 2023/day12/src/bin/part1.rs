fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, groups) = l.split_once(' ').unwrap();

            (
                springs.chars().collect::<Vec<_>>(),
                groups
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(count_arrangements)
        .sum()
}

fn replace_at_index(springs: &[char], c: char, idx: usize) -> Vec<char> {
    let mut clone = springs.to_vec();
    clone[idx] = c;
    clone
}

fn count_arrangements(record: (Vec<char>, Vec<usize>)) -> usize {
    let (springs, groups) = record;
    if let Some(idx) = springs.iter().position(|x| x == &'?') {
        let replace_working =
            count_arrangements((replace_at_index(&springs, '.', idx), groups.clone()));

        let replace_broken =
            count_arrangements((replace_at_index(&springs, '#', idx), groups.clone()));
        replace_broken + replace_working
    } else {
        let mut checked_groups: Vec<_> = vec![];
        let mut count = 0;
        for s in springs {
            match s {
                '#' => count += 1,
                '.' => {
                    if count > 0 {
                        checked_groups.push(count);
                        count = 0;
                    }
                }
                _ => panic!("unknown {s}"),
            }
        }
        if count > 0 {
            checked_groups.push(count);
        }


        if groups == checked_groups {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 21);
    }

    #[test]
    fn test_single() {
        assert_eq!(process("???.### 1,1,3"), 1);
        assert_eq!(process(".??..??...?##. 1,1,3"), 4);
        assert_eq!(process("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(process("????.#...#... 4,1,1"), 1);
        assert_eq!(process("????.######..#####. 1,6,5"), 4);
        assert_eq!(process("?###???????? 3,2,1"), 10);
    }
}
