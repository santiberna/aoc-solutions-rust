use std::collections::HashMap;

use crate::check_result;
use md5::Digest;

fn contains_a_triple(range: &[u8]) -> Option<u8> {
    range
        .windows(3)
        .find(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| w[0])
}

fn contains_matching_quintuplet(range: &[u8], val: u8) -> bool {
    range.windows(5).any(|w| w.iter().all(|&x| x == val))
}

fn md5_hash(str: &[u8]) -> [u8; 16] {
    md5::Md5::digest(str).into()
}

fn nibble_to_hex(n: u8) -> u8 {
    match n {
        0..=9 => b'0' + n,
        10..=15 => b'a' + (n - 10),
        _ => unreachable!(),
    }
}

fn md5_to_hex(hash: &[u8; 16]) -> [u8; 32] {
    let mut hex = [0u8; 32];

    for (i, byte) in hash.iter().enumerate() {
        hex[2 * i] = nibble_to_hex(byte >> 4);
        hex[2 * i + 1] = nibble_to_hex(byte & 0x0F);
    }

    hex
}

fn make_hash_input(salt: &[u8], num: usize) -> Vec<u8> {
    let mut out: Vec<u8> = salt.to_vec();
    out.extend_from_slice(num.to_string().as_bytes());
    out
}

#[derive(Default)]
struct HashCache {
    cache: HashMap<usize, [u8; 32]>,
}

impl HashCache {
    fn calculate<F>(&mut self, f: &F, index: usize) -> &[u8; 32]
    where
        F: Fn(usize) -> [u8; 32],
    {
        self.cache.entry(index).or_insert_with(|| f(index))
    }
}

fn algorithm<F>(f: &F) -> usize
where
    F: Fn(usize) -> [u8; 32],
{
    let mut cache = HashCache::default();
    let mut passwords = Vec::new();
    let mut index = 0;

    while passwords.len() < 64 {
        let result = cache.calculate(f, index);

        if let Some(m) = contains_a_triple(result) {
            for i in (index + 1)..=(index + 1000) {
                let result = cache.calculate(f, i);

                if contains_matching_quintuplet(result, m) {
                    passwords.push(index);
                    break;
                }
            }
        }

        index += 1;
    }

    *passwords.last().unwrap()
}

fn challenge(input: &[u8]) -> (usize, usize) {
    let hash1 = |u| {
        let hash_in = make_hash_input(input, u);
        let hash_out = md5_hash(&hash_in);
        md5_to_hex(&hash_out)
    };

    let hash2 = |u| {
        let hash_in = make_hash_input(input, u);

        let mut hash_out = md5_hash(&hash_in);

        for _ in 0..2016 {
            let rehash = md5_hash(&md5_to_hex(&hash_out));
            hash_out = rehash;
        }

        md5_to_hex(&hash_out)
    };

    (algorithm(&hash1), algorithm(&hash2))
}

//check_result!(b"yjdafjpo", 25427, 22045);
