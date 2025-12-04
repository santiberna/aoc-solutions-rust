use crate::check_result2;
use std::collections::HashSet;

fn map_ascii_char(item: &u8) -> usize {
    (if item.is_ascii_uppercase() {
        item.to_ascii_lowercase() - b'a' + 27
    } else {
        item - b'a' + 1
    }) as usize
}

fn find_intersect_item(s: &str) -> usize {
    let bytes = s.as_bytes();
    let (a, b) = bytes.split_at(bytes.len() / 2);

    let set: HashSet<_> = b.iter().collect();
    let item = a.iter().find(|&c| set.contains(c)).unwrap();

    map_ascii_char(item)
}

fn find_badge(triplet: &[&str]) -> usize {
    let mut final_set: HashSet<u8> = triplet[0].bytes().collect();

    for &s in &triplet[1..] {
        let set: HashSet<u8> = s.bytes().collect();
        final_set = final_set.intersection(&set).copied().collect();
    }

    assert!(final_set.len() == 1);
    map_ascii_char(final_set.iter().next().unwrap())
}

fn challenge() -> (usize, usize) {
    let input = crate::utility::input::get_input(2022, 3).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let sum_badge = lines.chunks(3).map(find_badge).sum();
    let sum_intersect = lines.into_iter().map(find_intersect_item).sum();

    (
        sum_intersect, //input.lines().map(calc_points1).sum(),
        sum_badge,     //input.lines().map(calc_points2).sum(),
    )
}

check_result2!(7990, 2602);
