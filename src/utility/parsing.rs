use regex::Regex;
use std::sync::LazyLock;

static DIGIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+").unwrap());

pub fn parse_all_numbers(line: &str) -> Vec<i64> {
    DIGIT_REGEX
        .find_iter(line)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}
