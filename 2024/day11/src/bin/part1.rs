use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 11).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> u64 {
    let mut stones = input.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect_vec();

    for n in  0..25 {
        stones = step(&stones);
        println!("{n}:{}", stones.len());
    }

    stones.len() as u64

}

fn step(stones: &[u64]) -> Vec<u64> {
let mut ret = vec![];

for n in stones {

    let digits= n.checked_ilog10().unwrap_or(0) + 1;
    if *n == 0 {
        ret.push(1);
    }
    else if digits % 2 == 0 {
        let bottom = n % 10u64.pow(digits /2);
        let top = n / 10u64.pow(digits /2);
        ret.extend([top, bottom]);
    } else {
        ret.push(n * 2024)
    }
}
ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(step(&[0,1,10,99,999]), vec![1,2024,1,0,9,9,2021976]);
    }
}
