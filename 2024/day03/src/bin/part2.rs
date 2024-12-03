use regex::Regex;

fn main() {
    let input: String = common::AocInput::fetch(2024, 3).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don\'t\(\))").unwrap();
    let mut mode = 1;
    let mut sum = 0;

    for (_, [path]) in re.captures_iter(input).map(|c| c.extract()) {
        match path {
            "do()" => mode = 1,
            "don't()" => mode = 0,
            _ => {
                let (a, b) = parse_mul(path).unwrap();
                sum += a * b * mode;
            }
        }
    }

    sum
}

fn parse_mul(input: &str) -> Option<(usize, usize)> {
    let (a, b) = input
        .strip_prefix("mul(")?
        .strip_suffix(")")?
        .split_once(",")?;

    let a = a.parse::<usize>().unwrap();
    let b = b.parse::<usize>().unwrap();
    Some((a, b))
}
