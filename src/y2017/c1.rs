use crate::{check_result2, utility};

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2017, 1).unwrap();

    let digits = input
        .trim()
        .bytes()
        .map(|c| (c - b'0') as usize)
        .collect::<Vec<_>>();

    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in 0..digits.len() {
        let val = digits[i];
        let next = digits[(i + 1) % digits.len()];
        let halfway = digits[(i + digits.len() / 2) % digits.len()];

        if val == next {
            answer1 += val
        }

        if val == halfway {
            answer2 += val;
        }
    }

    (answer1, answer2)
}

check_result2!(1251, 1244);
