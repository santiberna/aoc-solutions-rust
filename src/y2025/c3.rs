use crate::{check_result2, utility};

fn parse_chars(s: &str) -> Vec<i64> {
    s.as_bytes().iter().map(|c| (c - b'0') as i64).collect()
}

const TEST: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

fn make_number(digits: Vec<i64>) -> i64 {
    let mut scale = 1;
    let mut out = 0;
    for (_, d) in digits.iter().enumerate() {
        out += d * scale;
        scale *= 10;
    }
    out
}

fn pick_max_slow(slice: &[i64], remaining: usize) -> i64 {
    let mut max = 0;

    for element in 0..slice.len() - remaining {
        let next = if remaining > 0 {
            Some(pick_max_slow(&slice[element + 1..], remaining - 1))
        } else {
            None
        };
        let current = slice[element] * num::pow(10, remaining);
        max = max.max(next.unwrap_or(0) + current);
    }
    max
}

fn pick_max_fast(slice: &[i64], remaining: usize) -> i64 {
    let subslice = &slice[0..slice.len() - remaining];
    let first = subslice.iter().max().unwrap();
    let mut max = 0;

    for element in 0..slice.len() - remaining {
        if slice[element] == *first {
            let next = if remaining > 0 {
                Some(pick_max_fast(&slice[element + 1..], remaining - 1))
            } else {
                None
            };
            let current = first * num::pow(10, remaining);
            max = max.max(next.unwrap_or(0) + current);
        }
    }
    max
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 3).unwrap();
    let array = input.lines().map(parse_chars).collect::<Vec<_>>();
    let mut answer1 = 0;
    let mut answer2 = 0;

    // Challenge 1
    for line in array {
        let mut max = 0;
        for first in 0..line.len() {
            for second in first + 1..line.len() {
                let val = line[first] * 10 + line[second];
                max = max.max(val);
            }
        }

        answer1 += max;
        answer2 += pick_max_fast(&line, 11);
    }

    (answer1, answer2)
}

check_result2!(17330, 171518260283767);
