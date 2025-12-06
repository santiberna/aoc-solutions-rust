use std::collections::HashSet;

use crate::{check_result2, utility};

fn parse_range(r: &str) -> (i64, i64) {
    let (a, b) = r.split_at(r.find('-').unwrap());
    (a.parse().unwrap(), b.parse::<i64>().unwrap().abs())
}

fn is_fresh(n: i64, ranges: &[(i64, i64)]) -> bool {
    for (start, end) in ranges {
        if n >= *start && n <= *end {
            return true;
        }
    }
    return false;
}

fn merge_range(a: &(i64, i64), b: &(i64, i64)) -> Option<(i64, i64)> {
    let (a_start, a_end) = *a;
    let (b_start, b_end) = *b;
    if a_end < b_start || b_end < a_start {
        return None;
    }
    Some((a_start.min(b_start), a_end.max(b_end)))
}

fn find_overlap(set: &HashSet<(i64, i64)>) -> Option<((i64, i64), (i64, i64), (i64, i64))> {
    for a in set.iter() {
        for b in set.iter() {
            if &a == &b {
                continue;
            }
            if let Some(merged) = merge_range(a, b) {
                return Some((*a, *b, merged));
            }
        }
    }
    return None;
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 5).unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut answer1 = 0;

    let midpoint = lines.iter().position(|s| s.is_empty()).unwrap();

    let ranges = lines[0..midpoint]
        .iter()
        .map(|s| parse_range(*s))
        .collect::<Vec<_>>();

    let numbers = lines[midpoint + 1..]
        .iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    for n in numbers {
        if is_fresh(n, &ranges) {
            answer1 += 1;
        }
    }

    let mut set: HashSet<(i64, i64)> = ranges.into_iter().collect::<HashSet<_>>();

    while let Some((rem1, rem2, add)) = find_overlap(&set) {
        set.remove(&rem1);
        set.remove(&rem2);
        set.insert(add);
    }

    let answer2 = set.iter().fold(0, |acc, range| acc + 1 + range.1 - range.0);

    (answer1, answer2)
}

check_result2!(712, 332998283036769);
