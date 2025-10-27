use std::error::Error;

fn download_input(link: &str) -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let session = std::env::var("AOC_SESSION").expect("AOC_SESSION env variable not provided!");
    let client = reqwest::blocking::Client::new();

    let request = client
        .get(link)
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()
        .map_err(Box::new)?;

    Ok(request.text().map_err(Box::new)?)
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
        let path = format!("./input/Y{}_C{}.txt", year, challenge);
        std::fs::create_dir_all("./input")?;
        std::fs::write(path, &input).map_err(Box::new)?;

        Ok(input)
    }
}
