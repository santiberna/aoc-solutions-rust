use crate::check_result;
use nalgebra::Vector2;
use std::collections::HashSet;

type IVec2 = Vector2<i32>;

#[derive(Debug, Clone, Copy)]
enum Turn {
    LEFT,
    RIGHT,
}

 fn turn_right(pos: &IVec2) -> IVec2 {
            IVec2::new(pos.y, -pos.x)
            
        }

         fn turn_left(pos: &IVec2) -> IVec2 {
            IVec2::new(-pos.y, pos.x)
        }

fn parse_dir(elem: &str) -> (Turn, i32) {
    let (turn, number) = elem.split_at(1);

    match turn {
        "R" => (Turn::RIGHT, number.parse().unwrap()),
        "L" => (Turn::LEFT, number.parse().unwrap()),
        _ => todo!(),
    }
}

fn travel(dir: &mut IVec2, pos: &IVec2, (t, v): (Turn, i32)) -> IVec2 {
    match t {
        Turn::LEFT => {
            *dir = turn_left(dir);
            *pos + (*dir * v)
        }
        Turn::RIGHT => {
            *dir = turn_right(dir);
            *pos + (*dir * v)
        }
    }
}

fn track_travel(items: &Vec<(Turn, i32)>) -> IVec2 {
    let mut direction: IVec2 = IVec2::new(0, 1);
    let mut position: IVec2 = IVec2::default();

    let mut visited = HashSet::new();
    visited.insert(position);

    for (t, v) in items.iter() {
        match t {
            Turn::LEFT => direction = turn_left(&direction),
            Turn::RIGHT => direction = turn_right(&direction),
        }

        for _ in 0..*v {
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

    let mut direction: IVec2 = IVec2::new(0, 1);

    let position = items
        .iter()
        .fold(IVec2::new(0, 0), |acc, x| travel(&mut direction, &acc, *x));

    let repeat = track_travel(&items);

    (
        (position.x.abs() + position.y.abs()) as i64,
        (repeat.x.abs() + repeat.y.abs()) as i64,
    )
}

check_result!("input/Y2016/C1.txt", 226, 79);
