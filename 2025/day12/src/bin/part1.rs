use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2025, 12).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
struct Region {
    w: usize,
    h: usize,
    counts: [usize; 6],
}

impl Region {
    pub fn area(&self) -> usize {
        self.w * self.h
    }
}

fn process(input: &str) -> usize {
    let groups = input.split("\n\n").collect_vec();
    let shapes = &groups[0..groups.len() - 1];

    // packing in NP-hard.. but input is crafted so we just need to check areas :)
    let shapes: [usize; 6] = shapes
        .iter()
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .collect_vec()
        .try_into()
        .unwrap();

    let regions = groups[groups.len() - 1]
        .lines()
        .map(|l| {
            let (wh, counts) = l.split_once(": ").unwrap();
            let (w, h) = wh.split_once("x").unwrap();
            let counts = counts
                .splitn(6, ' ')
                .map(|d| d.parse::<usize>().unwrap())
                .collect_vec();

            Region {
                w: w.parse().unwrap(),
                h: h.parse().unwrap(),
                counts: counts.try_into().unwrap(),
            }
        })
        .collect_vec();

    regions
        .into_iter()
        .filter(|r| {
            let needed_area = r
                .counts
                .iter()
                .zip(shapes)
                .map(|(&c, s)| c * s)
                .sum::<usize>();
            r.area() > needed_area
        })
        .count()
}
