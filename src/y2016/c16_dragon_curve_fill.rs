use crate::check_result;

fn parse_bit(c: char) -> bool {
    match c {
        '1' => true,
        '0' => false,
        _ => unreachable!(),
    }
}

fn dragon_expansion(input: &mut Vec<bool>) {
    let copy = input.clone();

    input.push(false);
    input.extend(copy.iter().rev().map(|b| !b));
}

fn dragon_collapse(input: &mut Vec<bool>) {
    let new_vec: Vec<bool> = input
        .chunks(2)
        .map(|c| if c[0] == c[1] { true } else { false })
        .collect();
    *input = new_vec;
}

fn bits_to_string(bits: &Vec<bool>) -> String {
    bits.iter()
        .map(|b| if *b { '1' } else { '0' })
        .collect::<String>()
}

fn calc_checksum(input: &Vec<bool>, size: usize) -> Vec<bool> {
    let mut input = input.clone();

    while input.len() < size {
        dragon_expansion(&mut input);
    }

    input.resize(size, false);

    while input.len() % 2 == 0 {
        dragon_collapse(&mut input);
    }

    input
}

fn challenge(input: &str) -> (String, String) {
    let input: Vec<bool> = input.chars().map(|c| parse_bit(c)).collect();

    (
        bits_to_string(&calc_checksum(&input, 272)),
        bits_to_string(&calc_checksum(&input, 35651584)),
    )
}

//check_result!("10000", 0, 0);
check_result!(
    "01000100010010111",
    "10010010110011010".to_string(),
    "01010100101011100".to_string()
);
