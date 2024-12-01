use itertools::*;

fn main() {
    let input: String = common::AocInput::fetch(2015, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let Some((l, w, h)) = line
                .splitn(3, 'x')
                .map(|n| n.parse::<usize>().expect("Valid Int"))
                .collect_tuple()
            {
                let v = l * w * h;
                let mut sides = [l, w, h];
                sides.sort();
                let [a, b, _c] = sides;

                v + 2 * a + 2 * b
            } else {
                panic!("Expected 3 numbers")
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("2x3x4");
        assert_eq!(result, 34);
    }
}
