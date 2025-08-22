use std::cmp::Ordering;

use crate::check_result;

fn extract_range(str: &str) -> (u32, u32) {
    let (left, right) = str.split_at(str.find('-').unwrap_or(0));
    (left.parse().unwrap(), right[1..].parse().unwrap())
}

fn merge_range(range1: &(u32, u32), range2: &(u32, u32)) -> Option<(u32, u32)> {
    let (start1, end1) = *range1;
    let (start2, end2) = *range2;

    // Check if the ranges overlap or touch (inclusive range)
    let e1 = end1.checked_add(1).unwrap_or(end1);
    let e2 = end2.checked_add(1).unwrap_or(end2);

    if e1 >= start2 && start1 <= e2 {
        Some((start1.min(start2), end1.max(end2)))
    } else {
        None
    }
}

fn merge_ranges(mut vec: Vec<(u32, u32)>, range: &(u32, u32)) -> Vec<(u32, u32)> {
    if let Some(back) = vec.last_mut() {
        if let Some(result) = merge_range(back, range) {
            *back = result;
        } else {
            vec.push(*range);
        }
    } else {
        vec.push(*range);
    }

    vec
}

fn count_all(input: &Vec<(u32, u32)>) -> u32 {
    let mut current = u32::MAX;

    for &(a, b) in input {
        current -= (b - a + 1)
    }

    current + 1
}

fn challenge(input: &str) -> (u32, u32) {
    let mut ranges: Vec<(u32, u32)> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(extract_range)
        .collect();

    ranges.sort_by_key(|v| v.0);
    let merged_ranges = ranges.iter().fold(Vec::new(), merge_ranges);

    (merged_ranges[0].1 + 1, count_all(&merged_ranges))
}

check_result!("input/Y2016/C20.txt", 32259706, 113);
