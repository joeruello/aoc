use std::collections::VecDeque;

fn main() {
    let input: String = common::AocInput::fetch(2025, 10).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
struct Machine {
    pub target: u16,
    pub buttons: Vec<u16>,
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let lights = parts.next().unwrap();
            let _joltage = parts.next_back().unwrap();
            let buttons = parts
                .map(|c| {
                    dbg!(c)
                        .chars()
                        .filter_map(|c| {
                            let c = c.to_digit(10)?;
                            Some(2u16.pow(c))
                        })
                        .sum::<u16>()
                })
                .collect();

            let mut light_mask = 0;
            for (i, c) in lights.chars().enumerate() {
                if c == '#' {
                    light_mask += 1 << (i - 1);
                }
            }
            Machine {
                target: light_mask,
                buttons,
            }
        })
        .map(find_presses)
        .sum()
}

fn find_presses(machine: Machine) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((0, 0));

    while let Some((state, steps)) = queue.pop_front() {
        dbg!(&queue, state, steps);
        if state == machine.target {
            return steps;
        }

        machine
            .buttons
            .iter()
            .for_each(|b| queue.push_back((state ^ b, steps + 1)));
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 7);
    }
}
