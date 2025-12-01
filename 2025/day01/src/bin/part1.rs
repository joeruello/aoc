use core::panic;

fn main() {
    let input: String = common::AocInput::fetch(2025, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i32 {
    let rots: Vec<_> = input
        .lines()
        .map(|l| {
            let (dir, num) = l.split_at(1);
            let dir = match &dir.chars().next().unwrap() {
                'R' => 1,
                'L' => -1,
                _ => {
                    panic!("unknown direction {dir}")
                }
            };
            let num = num.parse::<i32>().unwrap();
            dir * num
        })
        .collect();

    let mut crossed = 0;
    let mut sum = 50;

    for n in rots {
        sum += n;
        if !(0..=99).contains(&sum) {
            sum = sum.rem_euclid(100);
            if sum == 0 {
                crossed += 1;
            }
        }
    }

    crossed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), 3);
    }
}
