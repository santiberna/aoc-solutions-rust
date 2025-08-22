use crate::{check_result2, utility};

fn parse_range(s: &str) -> ((usize, usize), (usize, usize)) {
    let (e1, e2) = s.split_once(',').unwrap();
    let (a1, a2) = e1.split_once('-').unwrap();
    let (b1, b2) = e2.split_once('-').unwrap();
    (
        (a1.parse().unwrap(), a2.parse().unwrap()),
        (b1.parse().unwrap(), b2.parse().unwrap()),
    )
}

fn eval_range(((a1, a2), (b1, b2)): ((usize, usize), (usize, usize))) -> usize {
    let a_inside = a1 >= b1 && a2 <= b2;
    let b_inside = b1 >= a1 && b2 <= a2;

    (a_inside || b_inside) as usize
}

fn eval_range2(((a1, a2), (b1, b2)): ((usize, usize), (usize, usize))) -> usize {
    (a1 <= b2 && b1 <= a2) as usize
}

fn challenge() -> (usize, usize) {
    let input: Vec<_> = utility::input::get_input(2022, 4)
        .unwrap()
        .lines()
        .map(parse_range)
        .collect();

    let sum1: usize = input.iter().copied().map(eval_range).sum();
    let sum2: usize = input.iter().copied().map(eval_range2).sum();

    (sum1, sum2)
}

check_result2!(466, 865);
