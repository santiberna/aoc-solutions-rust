use crate::check_result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_REGEX: Regex = Regex::new(
        r"^(?x)
        swap\ position\ (\d+)\ with\ position\ (\d+)
        |swap\ letter\ ([a-z])\ with\ letter\ ([a-z])
        |reverse\ positions\ (\d+)\ through\ (\d+)
        |rotate\ (left|right)\ (\d+)\ step(?:s)?
        |move\ position\ (\d+)\ to\ position\ (\d+)
        |rotate\ based\ on\ position\ of\ letter\ ([a-z])
        $"
    )
    .unwrap();
}

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    ReversePositions(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    MovePosition(usize, usize),
    RotateBasedOnLetter(u8),
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let caps = PARSE_REGEX.captures(line)?;

    Some(if let Some(pos1) = caps.get(1) {
        Instruction::SwapPosition(pos1.as_str().parse().unwrap(), caps[2].parse().unwrap())
    } else if let Some(ch1) = caps.get(3) {
        Instruction::SwapLetter(
            ch1.as_str().chars().next().unwrap() as u8,
            caps[4].chars().next().unwrap() as u8,
        )
    } else if let Some(pos1) = caps.get(5) {
        Instruction::ReversePositions(pos1.as_str().parse().unwrap(), caps[6].parse().unwrap())
    } else if let Some(dir) = caps.get(7) {
        let n = caps[8].parse().unwrap();
        match dir.as_str() {
            "left" => Instruction::RotateLeft(n),
            "right" => Instruction::RotateRight(n),
            _ => return None,
        }
    } else if let Some(pos1) = caps.get(9) {
        Instruction::MovePosition(pos1.as_str().parse().unwrap(), caps[10].parse().unwrap())
    } else if let Some(ch) = caps.get(11) {
        Instruction::RotateBasedOnLetter(ch.as_str().chars().next().unwrap() as u8)
    } else {
        return None;
    })
}

fn process(mut vec: Vec<u8>, instruction: &Instruction) -> Vec<u8> {
    match instruction {
        &Instruction::MovePosition(from, to) => {
            let v = vec.remove(from);
            vec.insert(to, v);
        }
        &Instruction::ReversePositions(start, end) => {
            vec[start..=end].reverse();
        }
        &Instruction::RotateBasedOnLetter(a) => {
            let pos = vec.iter().position(|&c| c == a).unwrap();
            if pos >= 4 {
                vec.rotate_right(pos + 2);
            } else {
                vec.rotate_right(pos + 1);
            }
        }
        &Instruction::RotateLeft(s) => vec.rotate_left(s),
        &Instruction::RotateRight(s) => vec.rotate_right(s),
        &Instruction::SwapLetter(a, b) => {
            let pos1 = vec.iter().position(|&c| c == a).unwrap();
            let pos2 = vec.iter().position(|&c| c == b).unwrap();
            vec.swap(pos1, pos2);
        }
        &Instruction::SwapPosition(a, b) => vec.swap(a, b),
    }

    vec
}

fn invert_rotate_based_on_letter(vec: Vec<u8>, target_char: u8) -> Vec<u8> {
    let len = vec.len();

    for i in 0..len {
        let mut candidate = vec.clone();
        candidate.rotate_left(i);

        // Apply the forward rule to candidate
        let pos = candidate.iter().position(|&c| c == target_char).unwrap();
        let mut rotated = candidate.clone();

        if pos >= 4 {
            rotated.rotate_right((pos + 2) % vec.len());
        } else {
            rotated.rotate_right((pos + 1) % vec.len());
        }

        if rotated == vec {
            return candidate; // found the original
        }
    }

    panic!("No inverse found!");
}

fn invert(mut vec: Vec<u8>, instruction: &Instruction) -> Vec<u8> {
    match instruction {
        &Instruction::MovePosition(from, to) => {
            let v = vec.remove(to);
            vec.insert(from, v);
        }
        &Instruction::ReversePositions(start, end) => {
            vec[start..=end].reverse();
        }
        &Instruction::RotateBasedOnLetter(a) => {
            return invert_rotate_based_on_letter(vec, a);
        }
        &Instruction::RotateLeft(s) => vec.rotate_right(s),
        &Instruction::RotateRight(s) => vec.rotate_left(s),
        &Instruction::SwapLetter(a, b) => {
            let pos1 = vec.iter().position(|&c| c == a).unwrap();
            let pos2 = vec.iter().position(|&c| c == b).unwrap();
            vec.swap(pos1, pos2);
        }
        &Instruction::SwapPosition(a, b) => vec.swap(a, b),
    }

    vec
}

fn challenge(input: &str) -> (String, String) {
    let instructions: Vec<Instruction> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_instruction)
        .map(|o| o.unwrap())
        .collect();

    let result = instructions.iter().fold(b"abcdefgh".to_vec(), process);
    let result2 = instructions.iter().rev().fold(b"fbgdceah".to_vec(), invert);

    (
        String::from_utf8(result).unwrap(),
        String::from_utf8(result2).unwrap(),
    )
}

check_result!(
    "input/C21.txt",
    "bfheacgd".to_string(),
    "gcehdbfa".to_string()
);
