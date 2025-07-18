use std::{cmp::Ordering, collections::HashMap};

use crate::check_result;

struct Room {
    pub encrypted_name: String,
    pub sector_id: i64,
    pub checksum: String,
}

fn trim_checksum(str: &str) -> String {
    let mut it = str.chars();
    it.next();
    it.next_back();
    it.collect()
}

fn parse_room(str: &str) -> Room {
    let (code, checksum) = str.split_at(str.find('[').unwrap());
    let (encrypted, num) = code.split_at(code.rfind('-').unwrap() + 1);

    let id: i64 = num.parse().unwrap_or(0);

    let trimmed_checksum = trim_checksum(checksum);

    Room {
        encrypted_name: encrypted.to_string(),
        sector_id: id,
        checksum: trimmed_checksum,
    }
}

fn sort_frequency((c1, f1): &(char, usize), (c2, f2): &(char, usize)) -> Ordering {
    let cmp = f2.cmp(f1);

    if cmp == Ordering::Equal {
        c1.cmp(c2)
    } else {
        cmp
    }
}

fn eval_room(room: &Room) -> bool {
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    for c in room.encrypted_name.chars().filter(|&c| c != '-') {
        *frequency_map.entry(c).or_insert(0) += 1
    }

    let mut frequency_array: Vec<(char, usize)> =
        frequency_map.iter().map(|(c, u)| (*c, *u)).collect();

    frequency_array.sort_by(sort_frequency);

    let mut result = true;

    for (i, c1) in room.checksum.chars().enumerate() {
        let (c2, _) = frequency_array[i];

        if c1 != c2 {
            result = false;
        }
    }

    result
}

fn decode_char(c: char, r: i64) -> char {
    if c == '-' {
        return ' ';
    }

    if c >= 'a' && c <= 'z' {
        let base = c as i64 - b'a' as i64 + r;
        let modulo = base % 26 + b'a' as i64;

        return modulo as u8 as char;
    }

    panic!()
}

fn decode_room_name(room: &Room) -> String {
    room.encrypted_name
        .chars()
        .map(|c| decode_char(c, room.sector_id))
        .collect()
}

fn challenge(input: &str) -> (i64, i64) {
    let rooms: Vec<Room> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_room)
        .collect();

    let fold_op = |acc: i64, room: &Room| {
        if eval_room(&room) {
            acc + room.sector_id
        } else {
            acc
        }
    };

    let part1 = rooms.iter().fold(0, fold_op);

    let correct_rooms: Vec<(String, i64)> = rooms
        .iter()
        .filter(|r| eval_room(r))
        .map(|r| (decode_room_name(r), r.sector_id))
        .collect();

    let part2 = correct_rooms
        .iter()
        .find(|(n, _)| n == "northpole object storage ")
        .map(|(_, i)| i)
        .cloned()
        .unwrap_or(0);

    (part1, part2)
}

check_result!("input/C4.txt", 137896, 501);
