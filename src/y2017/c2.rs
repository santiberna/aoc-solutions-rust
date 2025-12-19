use itertools::Itertools;

use crate::{
    check_result2,
    utility::{self, parsing::parse_all_numbers},
};

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2017, 2).unwrap();
    let numbers = input
        .lines()
        .map(parse_all_numbers::<usize>)
        .collect::<Vec<_>>();

    let mut answer1 = 0;
    let mut answer2 = 0;

    for vec in numbers.iter() {
        let max = *vec.iter().max().unwrap();
        let min = *vec.iter().min().unwrap();

        answer1 += max - min;

        for combination in vec.iter().combinations(2) {
            let max = **combination.iter().max().unwrap();
            let min = **combination.iter().min().unwrap();

            if max % min == 0 {
                answer2 += max / min;
            }
        }
    }

    (answer1, answer2)
}

check_result2!(1251, 1244);
