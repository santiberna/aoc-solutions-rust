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

fn find_variations(present: &MatrixVec<char>) -> Vec<MatrixVec<char>> {
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

fn parse_box(lines: &str) -> MatrixVec<char> {
    let separator = lines.find(':').unwrap();
    let line = &lines[separator + 1..];
    dbg!(line);
    MatrixVec::from_string(line.trim())
}

fn parse_target(line: &str) -> ([i64; 2], Vec<i64>) {
    let numbers = parse_all_numbers(line);
    (
        [numbers[0], numbers[1]],
        numbers.into_iter().skip(2).collect(),
    )
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 12).unwrap();

    let (boxes, targets) = {
        let sections = TEST.split("\n\n").map(str::trim).collect::<Vec<_>>();

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

        (dbg!(boxes), dbg!(targets))
    };

    {
        let variations = boxes.iter().map(find_variations).collect::<Vec<_>>();
        dbg!(variations);
    }

    let mut answer1 = 0;
    let mut answer2 = 0;

    (answer1, answer2)
}

check_result2!(0, 0);
