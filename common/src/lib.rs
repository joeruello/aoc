use anyhow::Result;
use reqwest::blocking::Client;
use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_downloads() {
        let result = AocInput::fetch(2023, 1).unwrap();
        println!("{:?}", result);
    }
}
