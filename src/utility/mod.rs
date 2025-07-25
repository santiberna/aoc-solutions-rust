use std::{array, default};

use num::{Num, Signed};
use md5::Digest;

#[macro_export]
macro_rules! check_result {
    ($in:expr, $a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge($in), ($a1, $a2))
        }
    };
}

pub fn positive_mod<T>(a: T, m: T) -> T
where
    T: Num + Copy + Signed,
{
    ((a % m) + m) % m
}

/// Computes gcd(a, b) and x, y such that a*x + b*y = gcd(a, b)
pub fn extended_gcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Num + Copy + Signed,
{
    if b == T::zero() {
        (a.abs(), a.signum(), T::zero())
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

/// Computes modular inverse of a mod m, if it exists
/// Returns Some(x) where (a * x) % m == 1, or None if inverse doesn't exist
pub fn modular_inverse<T>(a: T, m: T) -> Option<T> 
where
    T: Num + Copy + Signed
    {
    let (g, x, _) = extended_gcd(a, m);
    if g != T::one() {
        None // No inverse exists if a and m are not coprime
    } else {
        Some((x % m + m) % m) // ensure positive result
    }
}

pub fn md5_hash(str: &[u8]) -> [u8; 16] {
    md5::Md5::digest(str).into()
}

pub fn md5_to_hex(hash: &[u8; 16]) -> [u8; 32] {

    let nibble_to_hex = |n| {
        match n {
            0..=9 => b'0' + n,
            10..=15 => b'a' + (n - 10),
            _ => unreachable!(),
        }
    };

    let mut hex: [u8; 32] = <[u8; 32]>::default();

    for (i, byte) in hash.iter().enumerate() {
        hex[2 * i] = nibble_to_hex(byte >> 4);
        hex[2 * i + 1] = nibble_to_hex(byte & 0x0F);
    }

    hex
}
