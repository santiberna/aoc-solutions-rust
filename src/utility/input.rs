use std::{error::Error};

fn download_input(link: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;

    let session = std::env::var("AOC_SESSION").unwrap();
    let client = reqwest::blocking::Client::new();

    let request = client
        .get(link)
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()
        .map_err(Box::new)?;

    let text = request.text().map_err(Box::new)?;
    if text.starts_with("Puzzle inputs differ by user.  Please log in to get your puzzle input.") {
        Err("Session Code is expired or invalid".into())
    } else {
        Ok(text)
    }
}

fn read_cached_input(year: usize, challenge: usize) -> Option<String> {
    let path = format!("input/Y{}_C{}.txt", year, challenge);
    std::fs::read_to_string(path).ok()
}

pub fn get_input(year: usize, challenge: usize) -> Result<String, Box<dyn Error>> {
    if let Some(data) = read_cached_input(year, challenge) {
        Ok(data)
    } else {
        let link = format!("https://adventofcode.com/{}/day/{}/input", year, challenge);

        let input = download_input(&link)?;
        let path = format!("input/Y{}_C{}.txt", year, challenge);
        std::fs::write(path, &input).map_err(Box::new)?;

        Ok(input)
    }
}
