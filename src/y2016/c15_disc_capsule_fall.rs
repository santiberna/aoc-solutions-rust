use crate::{check_result, utility};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DISC_REGEX: Regex =
        Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=(\d+), it is at position (\d+).$")
            .unwrap();
}

fn into_equation(&(index, length, time, current): &(i64, i64, i64, i64)) -> (i64, i64) {
    (
        -utility::positive_mod(current - time + index, length),
        length,
    )
}

// (remainder, length)
fn parse_disc(s: &str) -> Option<(i64, i64)> {
    if let Some(capture) = DISC_REGEX.captures(s) {
        let index = capture[1].parse::<i64>().ok()?;
        let length = capture[2].parse::<i64>().ok()?;
        let time = capture[3].parse::<i64>().ok()?;
        let current = capture[4].parse::<i64>().ok()?;

        Some(into_equation(&(index, length, time, current)))
    } else {
        None
    }
}

fn constructive_crt(input: &Vec<(i64, i64)>) -> i64 {
    let product_mod = input.iter().fold(1i64, |acc, (_, l)| acc * (*l));
    let mut sum = 0;

    for &(remainder, modulus) in input.iter() {
        let mod_exclude = product_mod / modulus;
        let modular_inverse = utility::modular_inverse(mod_exclude, modulus).unwrap();

        sum += remainder * mod_exclude * modular_inverse
    }

    utility::positive_mod(sum, product_mod)
}

fn challenge(input: &str) -> (i64, i64) {
    let mut input: Vec<(i64, i64)> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|s| parse_disc(s).unwrap())
        .collect();

    let part1 = constructive_crt(&input);

    let new_disc = ((input.len() + 1) as i64, 11i64, 0i64, 0i64);
    input.push(into_equation(&new_disc));

    (part1, constructive_crt(&input))
}

check_result!("input/C15.txt", 376777, 3903937);
