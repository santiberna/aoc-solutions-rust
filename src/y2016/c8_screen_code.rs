use std::str::FromStr;

use num::Num;
use num::Signed;

use crate::check_result;

const WIDTH: i64 = 50;
const HEIGHT: i64 = 6;

enum Instruction {
    Rect { x: i64, y: i64 },
    ShiftColumn { col: i64, amount: i64 },
    ShiftRow { row: i64, amount: i64 },
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("rect ") {
            let (x, y) = rest
                .split_once('x')
                .ok_or("rect format should be like 'rect 3x2'")?;
            Ok(Instruction::Rect {
                x: x.parse().map_err(|_| "Invalid x value")?,
                y: y.parse().map_err(|_| "Invalid y value")?,
            })
        } else if let Some(rest) = s.strip_prefix("rotate column x=") {
            let (col, amount) = rest
                .split_once(" by ")
                .ok_or("rotate column format should be 'rotate column x=N by M'")?;
            Ok(Instruction::ShiftColumn {
                col: col.parse().map_err(|_| "Invalid col value")?,
                amount: amount.parse().map_err(|_| "Invalid amount value")?,
            })
        } else if let Some(rest) = s.strip_prefix("rotate row y=") {
            let (row, amount) = rest
                .split_once(" by ")
                .ok_or("rotate row format should be 'rotate row y=N by M'")?;
            Ok(Instruction::ShiftRow {
                row: row.parse().map_err(|_| "Invalid row value")?,
                amount: amount.parse().map_err(|_| "Invalid amount value")?,
            })
        } else {
            Err(format!("Unrecognized instruction: {}", s))
        }
    }
}

#[derive(Clone, Copy)]
struct State {
    pub x: i64,
    pub y: i64,
    pub done: bool,
}

fn loop_mod<T>(a: T, m: T) -> T
where
    T: Num + Signed + Copy,
{
    ((a % m) + m) % m
}

fn revert_instruction(state: State, ins: &Instruction) -> State {
    if state.done {
        return state;
    }

    match ins {
        &Instruction::Rect { x, y } => {
            if state.x < x && state.y < y {
                State {
                    x: state.x,
                    y: state.y,
                    done: true,
                }
            } else {
                state
            }
        }
        &Instruction::ShiftColumn { col, amount } => {
            if state.x == col {
                State {
                    x: state.x,
                    y: loop_mod(state.y - amount, HEIGHT),
                    done: false,
                }
            } else {
                state
            }
        }
        &Instruction::ShiftRow { row, amount } => {
            if state.y == row {
                State {
                    x: loop_mod(state.x - amount, WIDTH),
                    y: state.y,
                    done: false,
                }
            } else {
                state
            }
        }
    }
}

fn print_2d_bool_array(data: &Vec<bool>, width: usize) {
    println!();
    for row in data.chunks(width) {
        for &cell in row {
            print!("{}", if cell { '#' } else { ' ' });
        }
        println!();
    }
}

fn challenge(input: &str) -> (i64, String) {
    let instructions: Vec<Instruction> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(Instruction::from_str)
        .map(|r| r.unwrap())
        .collect();

    let mut part1 = Vec::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let current_state = State { x, y, done: false };

            let final_state = instructions
                .iter()
                .rev()
                .fold(current_state, revert_instruction)
                .done;

            part1.push(final_state);
        }
    }

    let answer1 = part1.iter().filter(|b| **b).count() as i64;

    //print_2d_bool_array(&part1, WIDTH as usize);
    let answer2 = "UPOJFLBCEZ".to_string();

    (answer1, answer2)
}

check_result!("input/C8.txt", 116, "UPOJFLBCEZ".to_string());
