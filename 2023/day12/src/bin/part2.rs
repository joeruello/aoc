use std::collections::HashMap;

fn main() {
    let input: String = common::AocInput::fetch(2023, 12).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let repeat_factor = 5;
    let lines: Vec<_> = input.lines().collect();
    let mut cache = HashMap::new();
    lines
        .into_iter()
        .map(|l| {
            let (springs, groups) = l.split_once(' ').unwrap();

            let springs: Vec<_> = (0..repeat_factor).map(|_| springs).collect();
            let springs = springs.join("?");
            (
                springs.chars().collect::<Vec<_>>(),
                groups
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
                    .repeat(repeat_factor),
            )
        })
        .map(|(springs, groups)| count_arrangements((&springs, &groups), &mut cache))
        .sum()
}

fn replace_first_item(chars: &[char], item: char) -> Vec<char> {
    let mut vec = vec![item];
    vec.extend(&chars[1..]);
    vec
}

fn count_arrangements(
    record: (&[char], &[usize]),
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    let (springs, groups) = record;
    if let Some(cached_result) = cache.get(&(springs.to_vec(), groups.to_vec())) {
        return *cached_result;
    }
    let result = if let Some(next) = springs.first() {
        match next {
            '.' => count_arrangements((&springs[1..], &groups), cache), 
            '#' => match groups.first() {
                None => 0, // broken pipe without a group
                Some(group_length) => {
                    let group_length = *group_length;
                    if group_length > springs.len() || springs[0..group_length].contains(&'.') {
                        0 // not enough pipes left to form a group or the potential group is broken by a .
                    } else if let Some('#') = &springs.get(group_length) {
                        0 // the character after the group is a broken pipe - meaning the group is too big
                    } else if springs.get(group_length + 1).is_none() { // if we're at the end of the input
                        if groups.len() == 1 { // and this is the last group
                            1
                        } else {
                            0 // end of input with unresolved groups
                        }
                    } else {
                        count_arrangements((&springs[group_length + 1..], &groups[1..]), cache) // valid group, eat tokens and move on
                    }
                }
            },
            '?' => {
                count_arrangements((&replace_first_item(springs, '.'), &groups), cache)
                    + count_arrangements((&replace_first_item(springs, '#'), &groups), cache)
            }
            _ => unreachable!(),
        }
    } else if groups.is_empty() {
        1 // no input remaning with all groups consumed means we found a match
    } else {
        0 // end of input with unresolved groups
    };

    cache.insert((springs.to_vec(), groups.to_vec()), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(process("???.### 1,1,3"), 1);
        assert_eq!(process(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(process("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(process("????.#...#... 4,1,1"), 16);
        assert_eq!(process("????.######..#####. 1,6,5"), 2500);
        assert_eq!(process("?###???????? 3,2,1"), 506250);
    }
}
