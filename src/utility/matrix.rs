#[derive(Clone, PartialEq)]
pub struct MatrixVec<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl MatrixVec<char> {
    pub fn from_string(str: &str) -> Self {
        let slices = str
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::from_slices(&slices)
    }
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

    pub fn from_slices<S: AsRef<[T]>>(data: &[S]) -> Self {
        let rows = data.first().unwrap().as_ref().len();
        let cols = data.len();
        let data = data.iter().fold(Vec::<T>::new(), |mut acc, slice| {
            acc.extend_from_slice(slice.as_ref());
            acc
        });
        Self::from_vec(rows, cols, data)
    }
}

impl<T> MatrixVec<T> {
    /// Immutable iterator over all elements in row-major order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Creates a new matrix from a Vec, panicking if the length is incorrect.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "Data length does not match dimensions"
        );
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
