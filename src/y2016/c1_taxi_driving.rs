use crate::check_result;
use crate::utility::math::Vec2;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Turn {
    LEFT,
    RIGHT,
}

fn parse_dir(elem: &str) -> (Turn, i32) {
    let (turn, number) = elem.split_at(1);

    match turn {
        "R" => (Turn::RIGHT, number.parse().unwrap()),
        "L" => (Turn::LEFT, number.parse().unwrap()),
        _ => todo!(),
    }
}

fn travel(dir: &mut Vec2<i32>, pos: &Vec2<i32>, (t, v): (Turn, i32)) -> Vec2<i32> {
    match t {
        Turn::LEFT => {
            *dir = dir.turn_left();
            *pos + (*dir * v)
        }
        Turn::RIGHT => {
            *dir = dir.turn_right();
            *pos + (*dir * v)
        }
    }
}

fn track_travel(items: &Vec<(Turn, i32)>) -> Vec2<i32> {
    let mut direction = Vec2::new(0, 1);
    let mut position: Vec2<i32> = Vec2::default();

    let mut visited = HashSet::new();
    visited.insert(position);

    for (t, v) in items.iter() {
        match t {
            Turn::LEFT => direction = direction.turn_left(),
            Turn::RIGHT => direction = direction.turn_right(),
        }

        for i in 0..*v {
            position = position + direction;
            let success = visited.insert(position);

            if success == false {
                return position;
            }
        }
    }

    panic!()
}

fn challenge(input: &str) -> (i64, i64) {
    let contents = std::fs::read_to_string(input).unwrap();
    //let contents = "R5, L5, R5, R3";

    let items: Vec<(Turn, i32)> = contents.split(", ").map(parse_dir).collect();

    let mut direction = Vec2::new(0, 1);

    let position = items
        .iter()
        .fold(Vec2::new(0, 0), |acc, x| travel(&mut direction, &acc, *x));

    let repeat = track_travel(&items);

    (
        (position.x.abs() + position.y.abs()) as i64,
        (repeat.x.abs() + repeat.y.abs()) as i64,
    )
}

check_result!("input/C1.txt", 226, 79);
