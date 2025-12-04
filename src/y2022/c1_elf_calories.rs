use crate::{check_result, utility};

fn sum_strs(strs: &[&str]) -> i64 {
    let sum = |acc, v: &&str| {
        if let Ok(i) = v.parse::<i64>() {
            acc + i
        } else {
            acc
        }
    };
    strs.iter().fold(0, sum)
}

fn challenge(_: &str) -> (i64, i64) {
    let file = utility::input::get_input(2022, 1).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut elfs: Vec<i64> = lines.split(|x| x.is_empty()).map(sum_strs).collect();
    elfs.sort();

    (
        elfs.iter().rev().take(1).sum(),
        elfs.iter().rev().take(3).sum(),
    )
}

check_result!("input/Y2022/C1.txt", 69281, 201524);
