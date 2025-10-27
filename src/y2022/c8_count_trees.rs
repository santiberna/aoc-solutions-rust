use crate::check_result2;
use crate::utility::matrix;
use nalgebra::Vector2;

type IVec2 = nalgebra::Vector2<i32>;

fn traverse_direction(
    grid: &matrix::MatrixVec<i32>,
    current: IVec2,
    direction: IVec2,
    source: i32,
) -> bool {
    let next_tree = current + direction;

    if next_tree.x <= 0 || next_tree.y <= 0 {
        true;
    }

    if let Some(value) = grid.get(next_tree.y as usize, next_tree.x as usize) {
        if *value >= source {
            false
        } else {
            traverse_direction(grid, next_tree, direction, source)
        }
    } else {
        true
    }
}

fn score_direction(
    grid: &matrix::MatrixVec<i32>,
    current: IVec2,
    direction: IVec2,
    source: i32,
) -> i32 {
    let next_tree = current + direction;

    if next_tree.x < 0 || next_tree.y < 0 {
        return 0;
    }

    if let Some(value) = grid.get(next_tree.y as usize, next_tree.x as usize) {
        if *value >= source {
            1
        } else {
            1 + score_direction(grid, next_tree, direction, source)
        }
    } else {
        0
    }
}

const TEST: &str = r#"30373
25512
65332
33549
35390"#;

fn challenge() -> (usize, usize) {
    let input: String = crate::utility::input::get_input(2022, 8).unwrap();
    //let input = TEST.to_string();

    let columns = input.as_str().lines().count();
    let rows = input.as_str().lines().next().unwrap().len();
    let input = input.replace('\n', "");

    let input: Vec<i32> = input
        .as_bytes()
        .iter()
        .map(|c| (*c - b'0') as i32)
        .collect();
    let grid = matrix::MatrixVec::from_vec(rows, columns, input);

    let mut sum = 0;
    let mut max = 0;

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let current = IVec2::new(x as i32, y as i32);
            let source = *grid.get(y, x).unwrap();
            let dirs = [
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ];

            let val = dirs
                .iter()
                .map(|d| traverse_direction(&grid, current, d.clone(), source))
                .any(|b| b);

            let score = dirs.iter().fold(1, |acc, d| {
                let score = score_direction(&grid, current, d.clone(), source);
                //println!("  Score: {score}");
                acc * score
            });

            //println!("{}, {} -> {}", x, y, score);
            sum += val as usize;
            max = max.max(score as usize);
        }
    }

    (sum, max)
}

check_result2!(1719, 590824);
