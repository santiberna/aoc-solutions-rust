use crate::check_result;
use crate::utility::math::Vec2;

fn parse_dir(c: char) -> Vec2<i32> {
    match c {
        'U' => Vec2::new(0, -1),
        'D' => Vec2::new(0, 1),
        'L' => Vec2::new(-1, 0),
        'R' => Vec2::new(1, 0),
        _ => panic!(),
    }
}

const PANEL: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const PANEL2: [char; 25] = [
    ' ', ' ', '1', ' ', ' ', ' ', '2', '3', '4', ' ', '5', '6', '7', '8', '9', ' ', 'A', 'B', 'C',
    ' ', ' ', ' ', 'D', ' ', ' ',
];

fn part1(items: &Vec<Vec<Vec2<i32>>>) -> i64 {
    let mut digits = Vec::new();
    let mut current = Vec2::new(1, 1);

    for row in items.iter() {
        for dir in row.iter() {
            current = current + *dir;

            current.x = current.x.clamp(0, 2);
            current.y = current.y.clamp(0, 2);
        }

        digits.push(PANEL[(current.x + current.y * 3) as usize]);
    }

    digits.iter().fold(0, |acc, &d| acc * 10 + d) as i64
}

fn part2(items: &Vec<Vec<Vec2<i32>>>) -> String {
    let mut code = String::new();
    let mut current = Vec2::new(0, 2);

    for row in items.iter() {
        for dir in row.iter() {
            let mut next = current + *dir;

            next.x = next.x.clamp(0, 4);
            next.y = next.y.clamp(0, 4);

            if PANEL2[(next.x + next.y * 5) as usize] != ' ' {
                current = next;
            }
        }

        code.push(PANEL2[(current.x + current.y * 5) as usize]);
    }

    code
}

fn challenge(input: &str) -> (i64, String) {
    let contents = std::fs::read_to_string(input).unwrap();
    let items: Vec<Vec<Vec2<i32>>> = contents
        .lines()
        .map(|line| line.chars().map(parse_dir).collect())
        .collect();

    (part1(&items), part2(&items))
}

check_result!("input/C2.txt", 78985, "57DD8".to_string());
