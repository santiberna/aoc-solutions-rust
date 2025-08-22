use std::collections::VecDeque;

use crate::{check_result2, utility};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn parse_state(lines: &[&str]) -> Vec<VecDeque<char>> {
    let stack_count = (lines.first().unwrap().len() + 2) / 4;
    let max_length = lines.len() - 1;

    let mut out = Vec::new();

    for i in 0..stack_count {
        let col = 1 + i * 4;
        let stack = (0..max_length)
            .rev()
            .fold(VecDeque::new(), |mut acc, index| {
                let char = lines.get(index).unwrap().chars().nth(col).unwrap();
                if char != ' ' {
                    acc.push_back(char)
                }
                acc
            });

        out.push(stack);
    }

    out
}

// count, src, dst
fn parse_commands(lines: &str) -> Option<(usize, usize, usize)> {
    if let Some(captures) = INSTRUCTION_REGEX.captures(lines) {
        Some((
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
            captures[3].parse().ok()?,
        ))
    } else {
        None
    }
}

fn get_top(state: &Vec<VecDeque<char>>) -> String {
    state.iter().fold(String::new(), |mut acc, s| {
        acc.push(*s.back().unwrap());
        acc
    })
}

fn part1(mut state: Vec<VecDeque<char>>, steps: &Vec<(usize, usize, usize)>) -> String {
    for (count, src, dst) in steps {
        for _ in 0..*count {
            let b = state[*src - 1].pop_back().unwrap();
            state[*dst - 1].push_back(b);
        }
    }

    get_top(&state)
}

fn part2(mut state: Vec<VecDeque<char>>, steps: &Vec<(usize, usize, usize)>) -> String {
    for (count, src, dst) in steps {
        let mut temp_stack = VecDeque::new();

        for _ in 0..*count {
            let b = state[*src - 1].pop_back().unwrap();
            temp_stack.push_back(b);
        }

        for b in temp_stack.into_iter().rev() {
            state[*dst - 1].push_back(b);
        }
    }

    get_top(&state)
}

fn challenge() -> (String, String) {
    let input: String = utility::input::get_input(2022, 5).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let (state, steps) = lines.split_at(lines.iter().position(|c| c.is_empty()).unwrap());

    let initial = parse_state(state);
    let steps = steps
        .iter()
        .skip(1)
        .map(|s| parse_commands(*s).unwrap())
        .collect::<Vec<_>>();

    (
        part1(initial.clone(), &steps),
        part2(initial.clone(), &steps),
    )
}

check_result2!("SVFDLGLWV".to_owned(), "DCVTCVPCL".to_owned());
