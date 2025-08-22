use crate::{check_result2, utility};
use std::collections::HashSet;
use std::hash::Hash;

const TEXT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

fn all_unique<I, T>(iter: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    let mut seen = HashSet::new();
    for item in iter {
        if !seen.insert(item) {
            return false;
        }
    }
    true
}

fn get_packet_start(s: &str) -> usize {
    s.as_bytes()
        .windows(4)
        .position(|slice| all_unique(slice))
        .unwrap()
}

fn get_message_start(s: &str) -> usize {
    s.as_bytes()
        .windows(14)
        .position(|slice| all_unique(slice))
        .unwrap()
}

fn challenge() -> (usize, usize) {
    let input: String = utility::input::get_input(2022, 6).unwrap();

    (get_packet_start(&input) + 4, get_message_start(&input) + 14)
}

check_result2!(1034, 2472);
