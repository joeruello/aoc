fn main() {
    let input: String = common::AocInput::fetch(2024, 7).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (res, operands) = l.split_once(": ").expect("parsed");
            (
                res.parse::<usize>().unwrap(),
                operands
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(res, operands)| guess_operator(*res, operands))
        .map(|(res, _)| res)
        .sum()
}

fn guess_operator(result: usize, nums: &[usize]) -> bool {
    let add = nums[0] + nums[1];
    let mul = nums[0] * nums[1];
    match nums.len() {
        2 => add == result || mul == result,
        _ => {
            guess_operator(result, &[&[add], &nums[2..]].concat())
                || guess_operator(result, &[&[mul], &nums[2..]].concat())
        }
    }
}
