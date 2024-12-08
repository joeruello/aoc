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
        .filter(|(res, operands)| guess_operator(*res, operands[0], &operands[1..]))
        .map(|(res, _)| res)
        .sum()
}

fn guess_operator(result: usize, head: usize, tail: &[usize]) -> bool {
    let add = head + tail[0];
    let mul = head * tail[0];
    let concat = concat(head, tail[0]);
    if let 1 = tail.len() {
        add == result || mul == result || concat == result
    } else {
        guess_operator(result, add, &tail[1..])
            || guess_operator(result, mul, &tail[1..])
            || guess_operator(result, concat, &tail[1..])
    }
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}
