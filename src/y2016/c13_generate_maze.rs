use crate::check_result;
use nalgebra::Vector2;
use std::collections::HashSet;
use std::collections::VecDeque;

type IVec2 = Vector2<i32>;

fn is_valid(pos: &IVec2, c: i32) -> bool {
    if pos.x < 0 || pos.y < 0 {
        false
    } else {
        let num = pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + c;
        (num as u32).count_ones() % 2 == 0
    }
}

pub const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
];

fn challenge(input: i32) -> (i64, i64) {
    let mut visited: HashSet<IVec2> = HashSet::new();
    let mut queue: VecDeque<(IVec2, i64)> = VecDeque::new();

    let start = IVec2::new(1, 1);
    let mut sub_50_counter = 0;

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((position, steps)) = queue.pop_front() {
        if steps <= 50 {
            sub_50_counter += 1
        }

        if position.x == 31 && position.y == 39 {
            return (steps, sub_50_counter);
        }

        for dir in DIRECTIONS.iter() {
            let check = position + dir;

            if !is_valid(&check, input) {
                continue;
            }

            if visited.contains(&check) {
                continue;
            }

            queue.push_back((check, steps + 1));
            visited.insert(check);
        }
    }

    (0, 0)
}

check_result!(1350, 92, 124);
