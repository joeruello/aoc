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
                let lw = l * w;
                let wh = w * h;
                let hl = h * l;

                let min = [lw, wh, hl]
                    .iter()
                    .min()
                    .expect("should have a min")
                    .to_owned();
                2 * lw + 2 * wh + 2 * hl + min
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
        assert_eq!(result, 58);
    }
}
