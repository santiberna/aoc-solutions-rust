use crate::{
    check_result2,
    utility::{
        self,
        directions::{self},
        matrix::MatrixVec,
    },
};

const TEST: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

fn get_2d(slice: &[u8], width: i64, height: i64, x: i64, y: i64) -> Option<u8> {
    if x >= width || x < 0 {
        None
    } else if y >= height || y < 0 {
        None
    } else {
        slice.get((y * width + x) as usize).cloned()
    }
}

fn get_removes(slice: &[u8], width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for y in 0..height as i64 {
        for x in 0..width as i64 {
            if get_2d(slice, width as i64, height as i64, x, y).unwrap() == b'@' {
                let directions = [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ];

                let values = directions
                    .map(|(x1, y1)| get_2d(slice, width as i64, height as i64, x + x1, y + y1));

                if values.iter().filter(|v| **v == Some(b'@')).count() < 4 {
                    out.push((x as usize, y as usize))
                }
            }
        }
    }
    out
}

fn get_removes2(data: &MatrixVec<char>) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for y in 0..data.rows() {
        for x in 0..data.cols() {
            if *data.get(x, y).unwrap() == '@' {
                let values = data.get_many(x, y, &directions::ALL);
                if values.iter().filter(|v| v.cloned() == Some('@')).count() < 4 {
                    out.push((x, y))
                }
            }
        }
    }
    out
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 4).unwrap();
    let mut data = MatrixVec::from_string(&input);

    let answer1 = get_removes2(&data);

    let mut answer2 = answer1.len();
    let mut start = answer1.clone();

    while !start.is_empty() {
        for (x, y) in &start {
            *data.get_mut(*x, *y).unwrap() = '.';
        }
        start = get_removes2(&data);
        answer2 += start.len();
    }

    (answer1.len(), answer2)
}

check_result2!(1549, 8887);
