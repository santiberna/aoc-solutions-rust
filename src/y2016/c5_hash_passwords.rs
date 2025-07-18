use crate::check_result;
use md5::Digest;

fn md5_hash(str: &[u8]) -> [u8; 16] {
    md5::Md5::digest(str).into()
}

fn check_zeros(res: &[u8; 16]) -> bool {
    res[0] == 0 && res[1] == 0 && (res[2] & 0xF0) == 0
}

fn get_sixth_hex(res: &[u8; 16]) -> u8 {
    (res[2] & 0x0F)
}

fn get_seventh_hex(res: &[u8; 16]) -> u8 {
    res[3] >> 4
}

fn u8_to_hex_string(digits: &[u8]) -> String {
    let transform = |n: u8| match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => panic!("Input must be between 0 and 15"),
    };

    digits.iter().copied().map(transform).collect()
}

fn challenge(input: &str) -> (String, String) {
    let mut digits_part1 = Vec::new();
    let mut digits_part2 = [None; 8];

    let mut index: i64 = 0;

    loop {
        let hash_in: Vec<u8> = input.bytes().chain(index.to_string().bytes()).collect();
        let hash = md5_hash(&hash_in);

        if check_zeros(&hash) {
            let hex6 = get_sixth_hex(&hash);

            if digits_part1.len() != 8 {
                digits_part1.push(hex6);
            }

            if hex6 < 8 && digits_part2[hex6 as usize].is_none() {
                let hex7 = get_seventh_hex(&hash);
                digits_part2[hex6 as usize] = Some(hex7);
            }
        }

        if digits_part2.iter().all(|o| o.is_some()) {
            break;
        }

        index += 1;
    }

    let part2: Vec<u8> = digits_part2
        .iter()
        .cloned()
        .map(|o| o.unwrap_or(16u8))
        .collect();

    (
        u8_to_hex_string(digits_part1.as_slice()),
        u8_to_hex_string(part2.as_slice()),
    )
}

//check_result!("ugkcyxxp", "d4cd2ee1".to_string(), "f2c730e5".to_string());
