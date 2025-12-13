use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

static DIGIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+").unwrap());

pub fn parse_all_numbers<T: FromStr>(line: &str) -> Vec<T> {
    DIGIT_REGEX
        .find_iter(line)
        .filter_map(|m| m.as_str().parse::<T>().ok())
        .collect()
}
