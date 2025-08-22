use crate::{check_result2, utility};

pub const STRATEGY: [[usize; 3]; 3] = [
    [3, 0, 6], // Rock vs Rock, Rock vs Paper, Rock vs Scissors
    [6, 3, 0], // Paper vs Rock, Paper vs Paper, Paper vs Scissors
    [0, 6, 3], // Scissors vs Rock, Scissors vs Paper, Scissors vs Scissors
];

pub const TEST: &str = r#"A Y
B X
C Z"#;

fn calc_points1(s: &str) -> usize {
    let opponent = match s.chars().next().unwrap() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => unreachable!(),
    };
    let you = match s.chars().next_back().unwrap() {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => unreachable!(),
    };

    STRATEGY[you][opponent] + you + 1
}

fn calc_points2(s: &str) -> usize {
    let opponent = match s.chars().next().unwrap() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => unreachable!(),
    };
    let you = match s.chars().next_back().unwrap() {
        'X' => (opponent + 2) % 3,
        'Y' => opponent,
        'Z' => (opponent + 1) % 3,
        _ => unreachable!(),
    };

    //println!("{}: {}", s, STRATEGY[you][opponent] + you + 1);

    STRATEGY[you][opponent] + you + 1
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2022, 2).unwrap();

    //dbg!(TEST.lines().map(calc_points2).sum::<usize>());

    (
        input.lines().map(calc_points1).sum(),
        input.lines().map(calc_points2).sum(),
    )
}

check_result2!(13221, 13131);
