use core::hash;
use std::collections::HashSet;

use crate::check_result2;

type IVec2 = nalgebra::Vector2<i32>;

fn orthogonal_decision(head: &IVec2, tail: &IVec2) -> Option<IVec2> {
    if head.x == tail.x + 2 {
        Some(tail + IVec2::new(1, 0))
    } else if head.x == tail.x - 2 {
        Some(tail + IVec2::new(-1, 0))
    } else if head.y == tail.y + 2 {
        Some(tail + IVec2::new(0, 1))
    } else if head.y == tail.y - 2 {
        Some(tail + IVec2::new(0, -1))
    } else {
        None
    }
}

fn diagonal_decision(head: &IVec2, tail: &IVec2) -> Option<IVec2> {
    if head.x == tail.x || head.y == tail.y {
        return None;
    }

    let x = if head.x > tail.x { 1 } else { -1 };
    let y = if head.y > tail.y { 1 } else { -1 };
    Some(tail + IVec2::new(x, y))
}

fn tail_follow(head: &IVec2, tail: &IVec2) -> IVec2 {
    for x in head.x - 1..=head.x + 1 {
        for y in head.y - 1..=head.y + 1 {
            if *tail == IVec2::new(x, y) {
                return tail.clone();
            }
        }
    }

    if head.x != tail.x && head.y != tail.y {
        diagonal_decision(head, tail).unwrap()
    } else {
        orthogonal_decision(head, tail).unwrap()
    }
}

fn parse_instruction(s: &str) -> Vec<IVec2> {
    let (dir, count) = s.split_at(s.find(' ').unwrap());
    let dir = match dir {
        "R" => IVec2::new(1, 0),
        "L" => IVec2::new(-1, 0),
        "U" => IVec2::new(0, 1),
        "D" => IVec2::new(0, -1),
        _ => panic!(),
    };

    vec![dir].repeat(count.trim().parse().unwrap())
}

fn challenge() -> (usize, usize) {
    let input = crate::utility::input::get_input(2022, 9).unwrap();
    let instructions = input.lines().fold(Vec::new(), |mut acc, s| {
        let s = parse_instruction(s);
        acc.extend(s);
        acc
    });

    let answer1 = {
        let mut hashset = HashSet::new();
        hashset.insert(IVec2::default());

        let mut head = IVec2::default();
        let mut tail = IVec2::default();

        for i in instructions.iter() {
            head += i;
            tail = tail_follow(&head, &tail);

            hashset.insert(tail.clone());
        }
        hashset.len()
    };

    let answer2 = {
        let mut hashset = HashSet::new();
        hashset.insert(IVec2::default());

        let mut head = IVec2::default();
        let mut rope = vec![IVec2::default()].repeat(9);

        for i in instructions.iter() {
            head += i;
            rope[0] = tail_follow(&head, &rope[0]);

            for i in 1..rope.len() {
                let result = {
                    let head = &rope[i - 1];
                    let tail = &rope[i];
                    tail_follow(head, tail)
                };

                rope[i] = result;
            }

            hashset.insert(rope.last().unwrap().clone());
        }

        hashset.len()
    };

    (answer1, answer2)
}

check_result2!(0, 0);
