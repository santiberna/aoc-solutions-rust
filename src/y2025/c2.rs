use crate::{
    check_result2,
    utility::{self, modular::get_divisors},
};

fn parse_range(r: &str) -> (i64, i64) {
    let (a, b) = r.split_at(r.find('-').unwrap());
    (a.parse().unwrap(), b.parse::<i64>().unwrap().abs())
}

const TEST: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

fn eval_number(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let (a, b) = s.split_at(s.len() / 2);
    a == b
}

fn eval_number2(n: i64) -> bool {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut divisors = get_divisors(chars.len());
    divisors.pop();

    for div in divisors {
        let first = &chars[0..div];
        let splits = chars.chunks(div).collect::<Vec<_>>();
        if splits.iter().all(|e| *e == first) {
            return true;
        }
    }
    false
}

#[test]
fn eleven() {
    assert_eq!(eval_number(11), true);
}

#[test]
fn range() {
    for i in 95..=115 {
        if eval_number2(i) {
            println!("{i}");
        }
    }
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 2).unwrap();
    let ranges = input.trim().split(',').map(parse_range).collect::<Vec<_>>();

    let mut answer1 = 0;
    let mut answer2 = 0;
    for (a, b) in ranges {
        let sum = (a..=b)
            .into_iter()
            .fold(0, |acc, i| if eval_number(i) { acc + i } else { acc });

        answer1 += sum;

        let sum = (a..=b)
            .into_iter()
            .fold(0, |acc, i| if eval_number2(i) { acc + i } else { acc });

        answer2 += sum;
    }

    (answer1, answer2)
}

check_result2!(5398419778, 15704845910);
