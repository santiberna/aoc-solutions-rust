use crate::{check_result2, utility};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Instruction {
    NoOp,
    Add(i32),
}

fn parse_instruction(s: &str) -> Vec<Instruction> {
    let mut words = s.split(' ');
    let first = words.next().unwrap();

    match first {
        "addx" => vec![
            Instruction::NoOp,
            Instruction::Add(words.next().map(i32::from_str).unwrap().unwrap()),
        ],
        "noop" => vec![Instruction::NoOp],
        _ => panic!(),
    }
}

fn get_register_value_during(instructions: &[Instruction], index: usize) -> i32 {
    1 + (0..index - 1)
        .map(|i| {
            if let Instruction::Add(v) = instructions[i] {
                v
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn wrap_every_n_chars(s: &str, n: usize) -> String {
    let mut out = String::with_capacity(s.len() + s.len() / n);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && i % n == 0 {
            out.push('\n');
        }
        out.push(c);
    }
    out
}

fn challenge() -> (usize, &'static str) {
    let input = utility::input::get_input(2022, 10).unwrap();

    let instructions = input.lines().fold(Vec::new(), |mut acc, str| {
        acc.extend(parse_instruction(str));
        acc
    });

    let sample_points = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for sample in sample_points {
        let sample_sum: i32 = get_register_value_during(&instructions, sample);
        sum += (sample_sum) * (sample) as i32;
    }

    let register_values = (1..=40 * 6)
        .map(|index| get_register_value_during(&instructions, index))
        .collect::<Vec<_>>();

    let pixels = register_values
        .iter()
        .enumerate()
        .map(|(index, register)| {
            let horizontal_pos = index % 40;
            (horizontal_pos as i32 - register).abs() < 2
        })
        .collect::<Vec<_>>();

    let pixels = pixels
        .iter()
        .map(|b| if *b { '#' } else { '.' })
        .collect::<String>();

    let wrapped = wrap_every_n_chars(&pixels, 40);
    println!("{}", wrapped);

    (sum as usize, "EHZFZHCZ")
}

check_result2!(14540, "EHZFZHCZ");

const TEST: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
