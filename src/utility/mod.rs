use num::{Num, Signed};
use md5::Digest;

pub mod assembunny;
pub mod input;

#[macro_export]
macro_rules! check_result {
    ($in:expr, $a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge($in), ($a1, $a2))
        }
    };
}

#[macro_export]
macro_rules! check_result2 {
    ($a1:expr, $a2:expr) => {
        #[test]
        fn check_results() {
            assert_eq!(challenge(), ($a1, $a2))
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

#[derive(Clone, PartialEq)]
pub struct MatrixVec<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> MatrixVec<T>
where
    T: Clone + Default,
{
    /// Creates a new matrix with the given dimensions, filled with the default value for T.
    pub fn new(rows: usize, cols: usize) -> Self {
        MatrixVec {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T> MatrixVec<T> {

    /// Immutable iterator over all elements in row-major order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Creates a new matrix from a Vec, panicking if the length is incorrect.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length does not match dimensions");
        MatrixVec { rows, cols, data }
    }


    /// Mutable iterator over all elements in row-major order
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    
    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Gets a reference to the element at (row, col).
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the element at (row, col).
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
    }

    /// Sets the value at (row, col).
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        } else {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for MatrixVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix {}x{}:", self.rows, self.cols)?;
        for r in 0..self.rows {
            write!(f, "  ")?; // indent
            for c in 0..self.cols {
                let val = &self.data[r * self.cols + c];
                write!(f, "{:?} ", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}