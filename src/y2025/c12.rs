use crate::{
    check_result2,
    utility::{self, matrix::MatrixVec, parsing::parse_all_numbers},
};

const TEST: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

pub enum Symmetry {
    None,
    Flip,
    Rotation,
}

fn find_variations(present: &MatrixVec<bool>) -> Vec<MatrixVec<bool>> {
    let mut permutations = vec![];
    for i in 0..4 {
        for f in 0..4 {
            let mut matrix = present.clone();

            if f == 1 {
                matrix = matrix.flip(true);
            } else if f == 2 {
                matrix = matrix.flip(false);
            } else if f == 3 {
                matrix = matrix.flip(true);
                matrix = matrix.flip(false);
            }

            for _ in 0..i {
                matrix = matrix.rotate()
            }

            let find = permutations.iter().find(|c| **c == matrix);

            if (find.is_none()) {
                permutations.push(matrix);
            }
        }
    }

    permutations
}

fn to_bool_matrix(mat: MatrixVec<char>) -> MatrixVec<bool> {
    let cols = mat.cols();
    let rows = mat.rows();
    let data = mat
        .into_iter()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    MatrixVec::from_vec(rows, cols, data)
}

fn parse_box(lines: &str) -> MatrixVec<bool> {
    let separator = lines.find(':').unwrap();
    let line = &lines[separator + 1..];

    let boxe = MatrixVec::from_string(line.trim());
    to_bool_matrix(boxe)
}

fn parse_target(line: &str) -> ([usize; 2], Vec<usize>) {
    let numbers = parse_all_numbers::<usize>(line);
    (
        [numbers[0], numbers[1]],
        numbers.into_iter().skip(2).collect(),
    )
}

fn box_fits(x: usize, y: usize, box_v: &MatrixVec<bool>, grid: &MatrixVec<bool>) -> bool {
    if y - 1 + box_v.rows() > grid.rows() || x - 1 + box_v.cols() > grid.cols() {
        return false;
    }

    for by in 1..box_v.rows() - 1 {
        for bx in 1..box_v.cols() - 1 {
            if *box_v.get(by, bx).unwrap() {
                if *grid.get(y + by, x + bx).unwrap() {
                    return false;
                }
            }
        }
    }

    true
}

fn place_box(
    x: usize,
    y: usize,
    box_v: &MatrixVec<bool>,
    grid: &MatrixVec<bool>,
) -> MatrixVec<bool> {
    let mut grid = grid.clone();

    for by in 1..box_v.rows() - 1 {
        for bx in 1..box_v.cols() - 1 {
            if *box_v.get(by, bx).unwrap() {
                *grid.get_mut(y + by, x + bx).unwrap() = true
            }
        }
    }

    grid
}

fn recursive_search(
    boxes: &Vec<Vec<MatrixVec<bool>>>,
    current_state: &MatrixVec<bool>,
    current_counts: &[usize],
) -> bool {
    let Some(next_box) = current_counts.iter().position(|v| *v != 0) else {
        return true;
    };

    let next_counts = {
        let mut c = current_counts.to_vec();
        c[next_box] -= 1;
        c
    };

    let picked_box = &boxes[next_box];

    for box_variation in picked_box.iter() {
        for y in 1..current_state.rows() - 1 {
            for x in 1..current_state.cols() - 1 {
                if *current_state.get(y, x).unwrap() {
                    continue;
                }

                if !box_fits(x, y, box_variation, current_state) {
                    continue;
                }

                let next_grid = place_box(x, y, box_variation, current_state);
                let recurse = recursive_search(boxes, &next_grid, &next_counts);

                if recurse {
                    return true;
                }
            }
        }
    }

    false
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 12).unwrap();

    let (boxes, targets) = {
        let sections = input.split("\n\n").map(str::trim).collect::<Vec<_>>();

        let boxes = sections[0..sections.len() - 1]
            .iter()
            .map(|s| parse_box(s))
            .collect::<Vec<_>>();

        let targets = sections
            .last()
            .unwrap()
            .lines()
            .map(parse_target)
            .collect::<Vec<_>>();

        (boxes, targets)
    };

    let mut answer1 = 0;
    let mut answer2 = 0;

    let box_variations = boxes.iter().map(find_variations).collect::<Vec<_>>();
    let box_areas = boxes
        .iter()
        .map(|v| v.iter().filter(|v| **v).count())
        .collect::<Vec<_>>();

    dbg!(&box_areas);
    dbg!(&box_variations);

    for (size, required) in targets {
        println!("{}x{}: {:?}", size[0], size[1], &required);

        // Cheap initial pruning
        let area = size[0] * size[1];
        let box_count = required.iter().sum::<usize>();

        if dbg!(area) >= dbg!(box_count * 9) {
            println!("Skipped big grid!");
            answer1 += 1;
            continue;
        }

        let total_box_size = required
            .iter()
            .zip(box_areas.iter())
            .map(|(a, b)| a * b)
            .sum::<usize>();

        if area < total_box_size {
            println!("Skipped small grid!");
            continue;
        }

        // We have been bamboozled
        unimplemented!()
        // let start_grid = MatrixVec::<bool>::new(size[1], size[0]);
        // answer1 += if recursive_search(&box_variations, &start_grid, &required) {
        //     1
        // } else {
        //     0
        // };
    }

    (answer1, answer2)
}

check_result2!(0, 0);
