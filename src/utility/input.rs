use std::{error::Error, fmt::format};

#[derive(Debug)]
pub struct FetchError {
    source: Box<dyn Error>,
}

impl FetchError {
    fn new<T>(e: T) -> Self
    where
        T: Error + 'static,
    {
        Self {
            source: Box::new(e),
        }
    }
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "InputFetchError: {}", &self.source)
    }
}
impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

fn download_input(link: &str) -> Result<String, FetchError> {
    let cookie = std::fs::read_to_string("session.txt").map_err(FetchError::new)?;
    let client = reqwest::blocking::Client::new();

    let request = client
        .get(link)
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .map_err(FetchError::new)?;

    request.text().map_err(FetchError::new)
}

fn read_cached_input(year: usize, challenge: usize) -> Option<String> {
    let path = format!("input/Y{}_C{}.txt", year, challenge);
    std::fs::read_to_string(path).ok()
}

pub fn get_input(year: usize, challenge: usize) -> Result<String, FetchError> {
    if let Some(data) = read_cached_input(year, challenge) {
        Ok(data)
    } else {
        let link = format!("https://adventofcode.com/{}/day/{}/input", year, challenge);

        let input = download_input(&link)?;
        let path = format!("input/Y{}_C{}.txt", year, challenge);
        std::fs::write(path, &input).map_err(FetchError::new)?;

        Ok(input)
    }
}
