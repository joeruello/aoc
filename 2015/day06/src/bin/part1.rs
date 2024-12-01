use itertools::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2015, 6).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

type Cordinate = (usize, usize);
type Instruction = (Action, Cordinate, Cordinate);

fn parse_cord(input: &str) -> (usize, usize) {
    input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_tuple()
        .expect("Proper cordinates")
}

const WIDTH: usize = 1000;

fn process(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let mut parts: Vec<_> = l.split_whitespace().collect();
            if *parts.first().unwrap() == "turn" {
                parts = parts[1..].to_vec();
            }

            let op = *parts.first().unwrap();
            let op = match op {
                "on" => Action::TurnOn,
                "off" => Action::TurnOff,
                "toggle" => Action::Toggle,
                _ => unreachable!(),
            };
            let a = *parts.get(1).unwrap();
            let a = parse_cord(a);
            let b = *parts.get(3).unwrap();
            let b = parse_cord(b);

            (op, a, b)
        })
        .collect();

    let mut lights = vec![false; WIDTH * WIDTH];

    for (action, (x1, y1), (x2, y2)) in instructions.into_iter() {
        for x in x1..=x2 {
            for y in y1..=y2 {
                let current = lights[(y * WIDTH) + x];
                lights[(y * WIDTH) + x] = match (&action, current) {
                    (Action::Toggle, true) => false,
                    (Action::Toggle, false) => true,
                    (Action::TurnOn, _) => true,
                    (Action::TurnOff, _) => false,
                }
            }
        }
    }

    lights.into_iter().filter(|l| *l).count()
}
