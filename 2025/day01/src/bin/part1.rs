fn main() {
    let input: String = common::AocInput::fetch(2025, 1).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> i32 {
    let rots: Vec<_> = input
        .lines()
        .map(|l| {
            let dir = match &l[..1] {
                "R" => 1,
                "L" => -1,
                _ => panic!("Unknown direction: {l}"),
            };
            let num = l[1..].parse::<i32>().unwrap();
            dir * num
        })
        .collect();

    let mut crossed = 0;
    let mut dial = 50;

    for n in rots {
        dial = (dial + n).rem_euclid(100);
        if dial == 0 {
            crossed += 1;
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
