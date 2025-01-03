pub use itertools::Itertools;
use reqwest::blocking::Client;
use std::io::prelude::*;
use std::path::Path;
use std::{env, fs, fs::File};
use toodee::{TooDee, TooDeeOps};
pub use {anyhow::Context, anyhow::Result};

#[derive(Debug, Clone)]
pub struct AocInput(String);

impl AocInput {
    pub fn fetch(year: i16, day: i16) -> Result<Self> {
        let path = format!("{year}/day{day:02}/input.txt");
        if Path::new(&path).exists() {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(Self(contents))
        } else {
            println!("Downloading input file: {}", path);
            let input = download_input(year, day)?;
            fs::write(&path, input.clone())?;
            Ok(Self(input))
        }
    }
}

impl From<AocInput> for String {
    fn from(val: AocInput) -> Self {
        val.0
    }
}

pub fn download_input(year: i16, day: i16) -> Result<String> {
    let client = Client::new();
    let session = env::var("AOC_SESSION_ID").expect("AOC_SEESSION_ID should be set");
    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("cookie", format!("session={session}"))
        .send()?;

    Ok(res.text_with_charset("utf8")?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub const CARDINALS: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

    pub fn bisect(&self, b: &Direction) -> Option<Direction> {
        match (self, b) {
            (Direction::N, Direction::E) | (Direction::E, Direction::N) => Some(Direction::NE),
            (Direction::N, Direction::W) | (Direction::W, Direction::N) => Some(Direction::NW),
            (Direction::S, Direction::E) | (Direction::E, Direction::S) => Some(Direction::SE),
            (Direction::S, Direction::W) | (Direction::W, Direction::S) => Some(Direction::SW),
            _ => None,
        }
    }

    pub fn rot180(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
            Direction::NE => Direction::SW,
            Direction::SE => Direction::NW,
            Direction::SW => Direction::NE,
            Direction::NW => Direction::SE,
        }
    }

    pub fn xy(&self) -> (isize, isize) {
        match self {
            Direction::NE => (1, -1),
            Direction::SE => (1, 1),
            Direction::SW => (-1, 1),
            Direction::NW => (-1, -1),
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }
}
impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        value.xy()
    }
}

pub trait DirectionOps<T> {
    fn move_point(
        &self,
        p: &(usize, usize),
        dir: impl Into<(isize, isize)>,
    ) -> Option<(usize, usize)>;

    fn find(&self, tile: T) -> Option<(usize, usize)>;

    fn neighbours(&self, point: (usize, usize)) -> impl Iterator<Item = (usize, usize)>;
}

impl<T: PartialEq> DirectionOps<T> for TooDee<T> {
    fn move_point(
        &self,
        (x0, y0): &(usize, usize),
        d: impl Into<(isize, isize)>,
    ) -> Option<(usize, usize)> {
        let (dy, dx) = d.into();
        let (width, height) = self.size();
        let x = x0.checked_add_signed(dx)?;
        let y = y0.checked_add_signed(dy)?;
        (x < width && y < height).then_some((x, y))
    }

    fn find(&self, tile: T) -> Option<(usize, usize)> {
        let (width, height) = self.size();

        (0..width)
            .cartesian_product(0..height)
            .find(|&p| self[p] == tile)
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let n = (y > 0).then_some((x, y.saturating_sub(1)));
        let s = (y < self.num_rows() - 1).then_some((x, y + 1));
        let w = (x > 0).then_some((x.saturating_sub(1), y));
        let e = (x < self.num_cols() - 1).then_some((x + 1, y));

        [n, e, s, w].into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_downloads() {
        let result = AocInput::fetch(2023, 1).unwrap();
        println!("{:?}", result);
    }
}
