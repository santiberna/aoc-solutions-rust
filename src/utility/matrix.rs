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
        let cols = data.first().unwrap().as_ref().len();
        let rows = data.len();
        let data = data.iter().fold(Vec::<T>::new(), |mut acc, slice| {
            acc.extend_from_slice(slice.as_ref());
            acc
        });
        Self::from_vec(rows, cols, data)
    }
}

impl<T> MatrixVec<T>
where
    T: Clone 
{
    /// Swaps the values in two rows
    pub fn swap_rows(&mut self, a: usize, b: usize) {
        let row2 = &self.data[b * self.cols..(b+1)*self.cols];
        let temp = row2.to_vec();

        for i in 0..self.cols {
            let v = self.get(a, i).unwrap().clone();
            *self.get_mut(b, i).unwrap() = v;
        }

        for (i, t) in temp.into_iter().enumerate() {
            *self.get_mut(a, i).unwrap() = t;
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

    /// Get an iterator for a specific row
    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < self.rows());
        let start = row*self.cols();
        let end = start + self.cols();
        self.data[start..end].iter()
    }

    /// Get an iterator for a specific row
    pub fn row_iter_mut(&mut self, row: usize) -> impl Iterator<Item = &mut T> {
        assert!(row < self.rows());
        let start = row*self.cols();
        let end = start + self.cols();
        self.data[start..end].iter_mut()
    }

    /// Get an iterator for a specific column
    pub fn col_iter(&self, col: usize) -> impl Iterator<Item = &T> {
        assert!(col < self.cols());
        ColumnIter { source: self, index: 0, column: col } 
    }

    /// Get an iterator for a specific column
    pub fn col_iter_mut(&mut self, col: usize) -> impl Iterator<Item = &mut T> {
        assert!(col < self.cols());
        ColumnIterMut { source: self, index: 0, column: col } 
    }
}

pub struct ColumnIter<'a, T> {
    source: &'a MatrixVec<T>,
    index: usize,
    column: usize,
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.source.get(self.index, self.column);
        self.index += 1;
        current
    }
}

pub struct ColumnIterMut<'a, T> {
    source: &'a mut MatrixVec<T>,
    index: usize,
    column: usize,
}

impl<'a, T> Iterator for ColumnIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.source.get_mut(self.index, self.column) {
            let reborrow = unsafe { &mut *(v as *mut T) };
            self.index += 1;
            Some(reborrow)
        } else {
            None
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
