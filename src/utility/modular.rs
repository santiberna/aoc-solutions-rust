use num::Num;
use num::Signed;

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
    T: Num + Copy + Signed,
{
    let (g, x, _) = extended_gcd(a, m);
    if g != T::one() {
        None // No inverse exists if a and m are not coprime
    } else {
        Some((x % m + m) % m) // ensure positive result
    }
}
