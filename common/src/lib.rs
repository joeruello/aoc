use reqwest::blocking::Client;
use std::env;

pub use anyhow::Result;
pub use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct AocInput(String);

impl AocInput {
    pub fn fetch(year: i16, day: i16) -> Result<Self> {
        let input = download_input(year, day)?;
        Ok(Self(input))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
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
