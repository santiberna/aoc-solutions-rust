use crate::{check_result2, utility};

fn parse_dir(elem: &str) -> i32 {
    let (turn, number) = elem.split_at(1);

    match turn {
        "R" => number.parse::<i32>().unwrap(),
        "L" => -number.parse::<i32>().unwrap(),
        _ => unimplemented!(),
    }
}

fn count_wraps(mut current: i32, delta: i32) -> usize {
    let mut wraps = 0;
    for _ in 0..delta.abs() {
        current = (current + delta.signum()).rem_euclid(100);

        if current == 0 {
            wraps += 1;
        }
    }
    wraps
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 1).unwrap();

    let numbers = input.lines().map(parse_dir).collect::<Vec<_>>();
    let mut current = 50;
    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in numbers {
        //println!("State: {}, Delta {}", current, i);

        let new = (current + i).rem_euclid(100);
        if current == 0 {
            answer1 += 1;
        }

        answer2 += count_wraps(current, i);
        current = new;
    }

    (answer1, answer2)
}

check_result2!(982, 6106);
