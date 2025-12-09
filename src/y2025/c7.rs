use crate::{
    check_result2,
    utility::{self, matrix::MatrixVec},
};

const TEST: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 7).unwrap();
    let grid = MatrixVec::from_string(&input);

    let mut answer1 = 0;

    {
        let mut grid = grid.clone();
        for r in 1..grid.rows() {
            for c in 0..grid.cols() {
                let up = *grid.get(r - 1, c).unwrap();
                if up != '|' && up != 'S' {
                    continue;
                }

                let tile = grid.get_mut(r, c).unwrap();

                match *tile {
                    '.' => {
                        *tile = '|';
                    }
                    '^' => {
                        let left = grid.get_mut(r, c - 1).unwrap();
                        *left = '|';
                        let right = grid.get_mut(r, c + 1).unwrap();
                        *right = '|';
                        answer1 += 1;
                    }
                    _ => {}
                }
            }

            //dbg!(&grid);
        }
    }

    let answer2;

    {
        let igrid = grid
            .iter()
            .map(|c| match *c {
                'S' => 1,
                '.' => 0,
                '^' => 0,
                _ => panic!(),
            })
            .collect::<Vec<i64>>();

        let mut igrid = MatrixVec::from_vec(grid.rows(), grid.cols(), igrid);

        for r in 1..grid.rows() {
            for c in 0..grid.cols() {
                let up = *igrid.get(r - 1, c).unwrap();
                if up == 0 {
                    continue;
                }

                let tile = grid.get(r, c).unwrap();

                match *tile {
                    '.' => {
                        *igrid.get_mut(r, c).unwrap() += up;
                    }
                    '^' => {
                        let left = igrid.get_mut(r, c - 1).unwrap();
                        *left += up;
                        let right = igrid.get_mut(r, c + 1).unwrap();
                        *right += up;
                    }
                    _ => {}
                }
            }
        }
        answer2 = igrid.row_iter(igrid.rows() - 1).sum();
    }

    (answer1, answer2)
}

check_result2!(1675, 187987920774390);
