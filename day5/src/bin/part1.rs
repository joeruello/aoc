use std::{collections::VecDeque, ops::RangeInclusive};

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

#[derive(Debug)]
struct Mapping {
    source: usize,
    dest: usize,
    range: usize,
}

impl Mapping {
    fn source_range(&self) -> RangeInclusive<usize> {
        self.source..=self.source + self.range
    }
    fn map(&self, input: usize) -> Option<usize> {
        match self.source_range().contains(&input) {
            true => Some(self.dest + (input - self.source)),
            false => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn map_source_to_dest(&self, src: usize) -> usize {
        for mapping in &self.mappings {
            if let Some(mapping) = mapping.map(src) {
                return mapping;
            }
        }
        src
    }
}

fn process(input: &str) -> usize {
    let mut parts: VecDeque<_> = input.split("\n\n").collect();

    let seeds: Vec<_> = parts
        .pop_front()
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|d| d.parse::<usize>().expect("Valid int"))
        .collect();

    let mappings: Vec<_> = parts
        .into_iter()
        .map(|part| {
            let mut lines: VecDeque<_> = part.lines().collect();
            let _ = lines.pop_front().expect("title").replace(':', "");
            let mappings: Vec<_> = lines
                .into_iter()
                .map(|line| {
                    let (destination, rest) = line.trim().split_once(' ').unwrap();
                    let (source, range) = rest.split_once(' ').unwrap();
                    Mapping {
                        source: source.parse::<usize>().unwrap(),
                        dest: destination.parse::<usize>().unwrap(),
                        range: range.parse::<usize>().unwrap(),
                    }
                })
                .collect();
            Map { mappings }
        })
        .collect();

    seeds
        .iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(*seed, |num, mapping| mapping.map_source_to_dest(num))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 35);
    }

    #[test]
    fn test_mapping() {
        let map = Map {
            mappings: vec![
                Mapping {
                    source: 10,
                    dest: 50,
                    range: 10,
                },
                Mapping {
                    source: 30,
                    dest: 60,
                    range: 10,
                },
            ],
        };
        assert_eq!(map.map_source_to_dest(15), 55);
        assert_eq!(map.map_source_to_dest(21), 21);
        assert_eq!(map.map_source_to_dest(30), 60);
        assert_eq!(map.map_source_to_dest(40), 70)
    }
}
