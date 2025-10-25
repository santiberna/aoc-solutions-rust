pub fn md5_hash(str: &[u8]) -> [u8; 16] {
    use md5::Digest;
    md5::Md5::digest(str).into()
}

pub fn md5_to_hex(hash: &[u8; 16]) -> [u8; 32] {
    let nibble_to_hex = |n| match n {
        0..=9 => b'0' + n,
        10..=15 => b'a' + (n - 10),
        _ => unreachable!(),
    };

    let mut hex: [u8; 32] = <[u8; 32]>::default();

    for (i, byte) in hash.iter().enumerate() {
        hex[2 * i] = nibble_to_hex(byte >> 4);
        hex[2 * i + 1] = nibble_to_hex(byte & 0x0F);
    }

    hex
}
