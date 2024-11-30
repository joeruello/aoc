use std::{collections::VecDeque, ops::RangeInclusive};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input: String = common::AocInput::fetch(2023, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
struct Mapping {
    source_range: RangeInclusive<usize>,
    dest: usize,
}

impl Mapping {
    fn map(&self, input: usize) -> Option<usize> {
        match self.source_range.contains(&input) {
            true => Some(self.dest + (input - self.source_range.start())),
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
        self.mappings.iter().find_map(|m| m.map(src)).unwrap_or(src)
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
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let first = *chunk.first().expect("Int");
            let width = *chunk.last().expect("Int");
            first..=(first + width)
        })
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
                    let source = source.parse::<usize>().unwrap();
                    let range = range.parse::<usize>().unwrap();
                    Mapping {
                        source_range: source..=source + range,
                        dest: destination.parse::<usize>().unwrap(),
                    }
                })
                .collect();
            Map { mappings }
        })
        .collect();

    let seeds: Vec<_> = seeds.into_iter().flatten().collect();

    seeds
        .par_iter()
        .progress()
        .map(|s| {
            let mut value = *s;
            for map in mappings.iter() {
                value = map.map_source_to_dest(value);
            }
            value
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 46);
    }
}
