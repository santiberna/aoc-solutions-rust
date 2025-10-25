use crate::check_result;
use crate::utility;
use nalgebra::Vector2;
use std::collections::VecDeque;

type IVec2 = Vector2<i64>;

const DIRECTIONS: [(u8, IVec2); 4] = [
    (b'U', IVec2::new(0, -1)),
    (b'D', IVec2::new(0, 1)),
    (b'L', IVec2::new(-1, 0)),
    (b'R', IVec2::new(1, 0)),
];

fn open_doors(input: &[u8]) -> [bool; 4] {
    let hash = utility::hash::md5_to_hex(&utility::hash::md5_hash(input));

    std::array::from_fn(|i| match hash[i] {
        b'0'..=b'a' => false,
        b'b'..=b'f' => true,
        _ => unreachable!(),
    })
}

fn find_shortest_path(input: &[u8]) -> Option<Vec<u8>> {
    let mut queue: VecDeque<(IVec2, Vec<u8>)> = VecDeque::new();
    queue.push_front((IVec2::new(0, 0), input.to_owned()));

    while let Some((pos, state)) = queue.pop_front() {
        if pos.x == 3 && pos.y == 3 {
            return Some(state);
        }

        let available_doors = open_doors(&state);

        let iter = available_doors
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(i, _)| i);

        for index in iter {
            let (step, dir) = DIRECTIONS[index];
            let new_pos = dir + pos;

            if new_pos.x < 0 || new_pos.x > 3 || new_pos.y < 0 || new_pos.y > 3 {
                continue;
            }

            let mut new_state = state.clone();
            new_state.push(step);

            queue.push_back((new_pos, new_state));
        }
    }

    None
}

fn find_longest_path(path: &mut Vec<u8>, pos: IVec2) -> usize {
    if pos.x == 3 && pos.y == 3 {
        return path.len();
    }

    let available_doors = open_doors(&path);
    let mut result = 0;

    for (index, &b) in available_doors.iter().enumerate() {
        if b == false {
            continue;
        }

        let (step, dir) = DIRECTIONS[index];
        let new_pos = pos + dir;

        if new_pos.x < 0 || new_pos.x > 3 || new_pos.y < 0 || new_pos.y > 3 {
            continue;
        }

        path.push(step);
        result = result.max(find_longest_path(path, new_pos));
        path.pop();
    }

    result
}

pub fn challenge(input: &[u8]) -> (String, usize) {
    let mut result1 = find_shortest_path(input).unwrap();
    result1.drain(..input.len());

    let mut input = input.to_vec();
    let result2 = find_longest_path(&mut input, IVec2::new(0, 0)) - input.len();

    (String::from_utf8(result1).unwrap(), result2)
}

check_result!(b"qljzarfv", "DRLRDDURDR".to_string(), 500);
