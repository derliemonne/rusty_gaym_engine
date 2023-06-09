use super::vector::Vector;
use std::fmt::{Display, Debug};
use std::iter::zip;
use std::ops::{Index, IndexMut, Add, Mul, Div, Sub, Neg};
use std::{fmt, ops, vec};

#[derive(Clone)]
pub struct Matrix<T: Clone> {
    rows: Vec<Vector<T>>,
    rows_count: usize,
    cols_count: usize,
}

impl<T> Matrix<T>
where T: Clone {
    /// Returns matrix with zero columns and zero rows.
    pub const fn empty() -> Matrix<T> {
        Matrix {
            rows: vec![],
            rows_count: 0,
            cols_count: 0,
        }
    }

    /// Return matrix with specified size and elements. 
    /// 
    /// # Examples
    /// ```
    /// # use rusty_gaym_engine::matrix::Matrix;
    /// let m = Matrix::new(3, 3, vec![
    ///     1.0, 2.0, 3.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0,
    /// ]);
    /// assert!(m.is_some());
    ///
    /// let m = Matrix::new(3, 3, vec![
    ///     1.0, 2.0, 3.0, 4.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0, 0.0, 10.0,
    /// ]);
    /// assert!(m.is_none());
    /// ```
    pub fn new_cloned(rows_count: usize, cols_count: usize, elements: Vec<T>) -> Option<Matrix<T>> {
        if rows_count * cols_count != elements.len() {
            return None;
        }
        Some(Matrix::from_rule(
            rows_count, 
            cols_count, 
            |i, j| elements.get(i * cols_count + j).unwrap().clone()))
    }

    /// Returns matrix with specified rows.
    /// 
    /// If `rows` is empty returns empty matrix.
    /// 
    /// Each row in `rows` is a `Vector`, all vectors must be the same size, otherwise function returns `None`.
    pub fn from_rows(rows: Vec<Vector<T>>) -> Option<Matrix<T>> {
        if rows.len() == 0 {
            return Some(Matrix::empty());
        }

        let rows_count = rows.len();
        let each_row_len: Vec<usize> = rows.iter().map(|vector| vector.dim()).collect();
        let first_row_len = each_row_len[0];
        let all_rows_are_the_same_len = each_row_len
            .iter()
            .all(|len| *len == first_row_len);

        if !all_rows_are_the_same_len {
            return None;
        }

        Some(Matrix {
            rows,
            rows_count,
            cols_count: first_row_len,
        })
    }

    /// Returns matrix with specified columns.
    /// 
    /// If `cols` is empty returns empty matrix.
    /// 
    /// Each column in `cols` is a `Vector`, all vectors must be the same size, otherwise function returns `None`.
    pub fn from_cols(cols: Vec<Vector<T>>) -> Option<Matrix<T>> {
        Some(Matrix::from_rows(cols)?.transposed())
    }

    /// Returns matrix made of one row.
    pub fn from_row(row: Vector<T>) -> Matrix<T> {
        let cols_count = row.dim();
        Matrix {
            rows: vec![row],
            rows_count: 1,
            cols_count
        }
    }

    /// Returns matrix made of one column.
    pub fn from_col(col: Vector<T>) -> Matrix<T> {
        Matrix::from_row(col).transposed()
    }

    /// Creates and returns new Matrix instance with the specified number of rows and columns, 
    /// using the provided closure `f` to initialize each element in the matrix.
    /// 
    /// 
    /// # Arguments
    /// 
    /// * `rows_count` - The number of rows in the matrix
    /// * `cols_count` - The number of columns in the matrix
    /// * `f` - A closure that takes in two usize variables representing the row and column 
    ///         indices of an element, and returns an f32 value to initialize that element.
    /// 
    ///  If `rows_count` or `cols_count` is `0` empty matrix will be returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rusty_gaym_engine::matrix::Matrix;
    /// # use rusty_gaym_engine::vector::Vector;
    /// let matrix = Matrix::from_rule(3, 4, |i, j| (i + j) as f32);
    /// 
    /// assert_eq!(matrix, Matrix::from_rows(vec![
    ///     Vector::new(vec![0.0, 1.0, 2.0, 3.0]),
    ///     Vector::new(vec![1.0, 2.0, 3.0, 4.0]),
    ///     Vector::new(vec![2.0, 3.0, 4.0, 5.0])
    /// ]).unwrap());
    /// ```
    pub fn from_rule<Function>(rows_count: usize, cols_count: usize, f: Function) -> Matrix<T>
    where
        Function: Fn(usize, usize) -> T,
    {
        if rows_count == 0 || cols_count == 0 {
            return Matrix::empty()
        }

        let mut rows = vec![];
        for i in 0..rows_count {
            let mut row = vec![];
            for j in 0..cols_count {
                row.push(f(i, j));
            }
            let row = Vector::new(row);
            rows.push(row);
        }

        Matrix { rows, rows_count, cols_count }
    }

    pub fn get_minor(&self, rows_for_exclusion: Vec<usize>, cols_for_exclusion: Vec<usize>) -> Matrix<T> {
        let included_rows = (0..self.rows_count)
            .filter(|i| !rows_for_exclusion.contains(i))
            .map(|i| self[i].clone())
            .collect();
        let matrix_reduced_rows = Matrix::from_rows(included_rows).unwrap();
        let included_cols = (0..matrix_reduced_rows.cols_count)
            .filter(|i| !cols_for_exclusion.contains(i))
            .map(|i| matrix_reduced_rows.get_col(i).unwrap())
            .collect();
        Matrix::from_cols(included_cols).unwrap()
    }

    pub fn rows_count(&self) -> usize {
        self.rows_count
    }

    pub fn cols_count(&self) -> usize {
        self.cols_count
    }

    pub fn get_row(&self, index: usize) -> Option<Vector<T>> {
        self.rows.get(index).map(|row| row.clone())
    }

    pub fn get_col(&self, index: usize) -> Option<Vector<T>> {
        self.transposed().get_row(index)
    }
    
    pub fn transposed(&self) -> Matrix<T> {
        Matrix::from_rule(self.cols_count, self.rows_count, |i, j| self[j][i].clone())
    }

    pub fn multiply(&self, other: &Matrix<T>) -> Option<Matrix<T>>
    where T: Add<Output = T> + Mul<Output = T> + std::iter::Sum + Copy {
        if self.cols_count != other.rows_count {
            return None;
        }
        Some(Matrix::from_rule(self.rows_count, other.cols_count, |i, j| {
            (0..self.cols_count)
                .map(|t: usize| self[i][t] * other[t][j])
                .sum()
        }))
    }

    pub fn multiply_by_vector(&self, other: &Vector<T>) -> Option<Vector<T>>
    where T: Copy + Add<Output = T> + Mul<Output = T> + std::iter::Sum {
        let vector_len = other.dim();
        let result: Matrix<T> = (self * &Matrix::from_col(other.clone()))?;
        // assert that result matrix is a vector
        debug_assert!(result.cols_count == 1 && result.rows_count == vector_len);
        let result: Vector<T> = result.get_col(0).unwrap();
        Some(result)
    }

    pub fn set_row(&mut self, row_index: usize, new_row: Vector<T>) -> Result<(), ()> {
        if new_row.dim() != self.rows_count {
            return Err(());
        }

        match self.rows.get_mut(row_index) {
            Some(row) => *row = new_row,
            None => return Err(()),
        }

        Ok(())
    }

    pub fn set_col(&mut self, col_index: usize, new_col: Vector<T>) -> Result<(), ()> {
        if col_index >= self.cols_count {
            return Err(());
        }

        if new_col.dim() != self.cols_count {
            return Err(());
        }

        for i in 0..self.cols_count {
            self.rows[i].set(col_index, new_col[i].clone())?;
        }

        Ok(())
    }

    pub fn set(&mut self, row_index: usize, col_index: usize, value: T) -> Result<(), ()> {
        let row = match self.rows.get_mut(row_index) {
            None => return Err(()),
            Some(row) => row,
        };

        let x = match row.elements.get_mut(col_index) {
            None => return Err(()),
            Some(x) => x,
        };

        *x = value;

        Ok(())
    }

    pub fn swap_cols(&mut self, a_index: usize, b_index: usize) -> Result<(), ()> {
        let a_col = self.get_col(a_index).ok_or(())?;
        let b_col = self.get_col(b_index).ok_or(())?;

        self.set_col(a_index, b_col)?;
        self.set_col(b_index, a_col)?;

        Ok(())
    }

    pub fn swap_rows(&mut self, a_index: usize, b_index: usize) -> Result<(), ()> {
        let a_row = self.get_row(a_index).ok_or(())?;
        let b_row = self.get_row(b_index).ok_or(())?;

        self.set_row(a_index, b_row)?;
        self.set_row(b_index, a_row)?;

        Ok(())
    }
}

impl<Copyable> Matrix<Copyable> 
where Copyable: Copy {
    pub fn get(&self, row_index: usize, col_index: usize) -> Option<Copyable> {
        self.get_row(row_index)?.get(col_index)
    }

    /// Return matrix with specified size and elements. 
    /// 
    /// # Examples
    /// ```
    /// # use rusty_gaym_engine::matrix::Matrix;
    /// let m = Matrix::new(3, 3, vec![
    ///     1.0, 2.0, 3.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0,
    /// ]);
    /// assert!(m.is_some());
    ///
    /// let m = Matrix::new(3, 3, vec![
    ///     1.0, 2.0, 3.0, 4.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0, 0.0, 10.0,
    /// ]);
    /// assert!(m.is_none());
    /// ```
    pub fn new(rows_count: usize, cols_count: usize, elements: Vec<Copyable>) -> Option<Matrix<Copyable>> {
        if rows_count * cols_count != elements.len() {
            return None;
        }
        Some(Matrix::from_rule(
            rows_count, 
            cols_count, 
            |i, j| elements[i * cols_count + j]))
    }
}

impl Matrix<f32> {
    /// Returns an identity matrix of the specified size.
    /// The identity matrix is a square matrix with ones on the diagonal and zeroes elsewhere.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the square matrix. If `size` is zero, empty matrix will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use rusty_gaym_engine::matrix::Matrix;
    /// let identity = Matrix::identity(3);
    /// assert_eq!(identity, Matrix::new(3, 3, vec![1.0, 0.0, 0.0,
    ///                                             0.0, 1.0, 0.0,
    ///                                             0.0, 0.0, 1.0]).unwrap());
    /// ```
    pub fn identity(size: usize) -> Matrix<f32> {
        Matrix::from_rule(size, size, |i, j| 
        if i == j {
            1.0
        } else {
            0.0
        })
    }

    pub fn zeroes(rows_count: usize, cols_count: usize) -> Matrix<f32> {
        let rows = vec![Vector::new(vec![0.0; cols_count]); rows_count];
        Matrix { rows_count, cols_count, rows }
    }

    pub fn gram_matrix(bases: Vec<Vector<f32>>) -> Matrix<f32> {
        Matrix::from_rule(bases.len(), bases.len(), |i, j| {
            bases[i].dot_product(&bases[j])
        })
    }

    pub fn adjoint_matrix(&self) -> Option<Matrix<f32>> {
        self.determinant()?;
        let adjoint = Matrix::from_rule(self.rows_count, self.cols_count, |i, j|
            if (i + j) % 2 == 0 {1.0} else {-1.0} * self.get_minor(vec![i], vec![j]).determinant().unwrap())
            .transposed();
        
        Some(adjoint)
    }

    pub fn rotation_matrix2d(radians: f32) -> Matrix<f32> {
        Matrix::from_rows(vec![
            Vector::new(vec![radians.cos(), -radians.sin()]),
            Vector::new(vec![radians.sin(), radians.cos()])
        ]).unwrap()
    }

    pub fn rotation_matrix3d(x_radians: f32, y_radians: f32, z_radians: f32) -> Matrix<f32> {
        let x_rotation = Matrix::from_rows(vec![
            Vector::new(vec![1.0, 0.0, 0.0]),
            Vector::new(vec![0.0, x_radians.cos(), -x_radians.sin()]),
            Vector::new(vec![0.0, x_radians.sin(), x_radians.cos()])
        ]).unwrap();
        let y_rotation = Matrix::from_rows(vec![
            Vector::new(vec![y_radians.cos(), 0.0, y_radians.sin()]),
            Vector::new(vec![0.0, 1.0, 0.0]),
            Vector::new(vec![-y_radians.sin(), 0.0, y_radians.cos()])
        ]).unwrap();
        let z_rotation = Matrix::from_rows(vec![
            Vector::new(vec![z_radians.cos(), -z_radians.sin(), 0.0]),
            Vector::new(vec![z_radians.sin(), z_radians.cos(), 0.0]),
            Vector::new(vec![0.0, 0.0, 1.0])
        ]).unwrap();
        ((x_rotation * y_rotation).unwrap() * z_rotation).unwrap()
    }

    /// Returns determinant of a square matrix. If matrix is non-square return `None`.
    /// 
    /// Algorithm complexity is O(n^3) where n is the order of matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_gaym_engine::matrix::Matrix;
    /// let m = Matrix::new(3, 3, vec![
    ///     1.0, 2.0, 3.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0,
    /// ]).unwrap();
    ///
    /// let det = m.determinant().unwrap();
    /// 
    /// assert!(det.abs() < 1e-7);
    /// ```
    pub fn determinant(&self) -> Option<f32> {
        if self.cols_count != self.rows_count {
            return None;
        }

        let mut sign: i32 = 1;
        let mut matrix = self.clone(); 
        let matrix_size = matrix.cols_count();

        for i in 0..matrix_size {
            if matrix[i][i].abs() < f32::EPSILON {
                for j in (i + 1)..(matrix_size) {
                    if matrix[j][i].abs() > f32::EPSILON {
                        matrix.swap_rows(i, j).unwrap();
                        sign *= -1;
                        println!("{:?}", matrix);
                        break;
                    }
                }
                return Some(0.0);
            }

            for j in (i + 1)..matrix_size {
                let row_i = matrix.get_row(i).unwrap();
                let row_j = matrix.get_row(j).unwrap();
                let row_j = (row_j - (&row_i * (matrix[j][i] / matrix[i][i]))).unwrap();

                matrix[j] = row_j;

                println!("{:?}", matrix);
            }
        }

        println!("{:?}", matrix);

        // Now matrix is ready to count its determinant by multiplying all elements on main diagonal.
        let mut determinant: f32 = sign as f32;
        for i in 0..matrix_size {
            determinant *= matrix[i][i];
        }

        Some(determinant)
    }

    pub fn inverse(&self) -> Option<Matrix<f32>> {
        let determinant = self.determinant()?;
        if determinant.abs() < f32::EPSILON {
            return None
        }

        let adjoint = self.adjoint_matrix().unwrap();

        Some(1.0 / determinant * adjoint)
    }

    pub fn approximately_equal(&self, other: &Matrix<f32>, epsilon: f32) -> bool {
        if self.cols_count != other.cols_count || self.rows_count != other.rows_count {
            return false
        }

        for (row1, row2) in zip(self.rows.iter(), other.rows.iter()) {
            if !row1.approximately_equal(&row2, epsilon) {
                return false;
            }
        }

        return true;
    }
}

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = Vector<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl<T> PartialEq for Matrix<T>
where T: PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool {
        if self.cols_count != other.cols_count || self.rows_count != other.rows_count {
            return false
        }
        for (row1, row2) in zip(self.rows.iter(), other.rows.iter()) {
            if row1 != row2 {
                return false
            }
        }
        true
    }
}

impl<T> Add<Matrix<T>> for Matrix<T> 
where T: Add<Output = T> + Clone {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.cols_count != rhs.cols_count || self.rows_count != rhs.rows_count {
            return None;
        }

        Matrix::from_rows(
            self.rows
                .into_iter()
                .zip(rhs.rows.into_iter())
                .map(|(self_row, rhs_row)| (self_row + rhs_row).unwrap())
                .collect()

        )
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Matrix<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = self.clone();
        for col in matrix.rows.iter_mut() {
            *col *= rhs;
        }
        matrix
    }
}

impl Mul<Matrix<f32>> for f32 {
    type Output = Matrix<f32>;

    fn mul(self, rhs: Matrix<f32>) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Matrix<f32> {
    type Output = Matrix<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<T> Neg for Matrix<T> 
where T: Copy + Neg<Output = T> {
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Matrix::from_rule(
            self.rows_count, 
            self.cols_count, 
            |i, j| -self[i][j])
    }
}

impl<T> Sub<Matrix<T>> for Matrix<T>
where T: Copy + Neg<Output = T> + Add<Output = T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        self + -rhs
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where T: Copy + Add<Output = T> + Mul<Output = T> + std::iter::Sum {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self.multiply(&rhs)
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where T: Copy + Add<Output = T> + Mul<Output = T> + std::iter::Sum {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        self.multiply(rhs)
    }
}

impl<T> Mul<Vector<T>> for Matrix<T>
where T: Copy + Add<Output = T> + Mul<Output = T> + std::iter::Sum {
    type Output = Option<Vector<T>>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self.multiply_by_vector(&rhs)
    }
}

impl<T> Mul<&Vector<T>> for &Matrix<T>
where T: Copy + Add<Output = T> + Mul<Output = T> + std::iter::Sum {

    type Output = Option<Vector<T>>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        self.multiply_by_vector(rhs)
    }
}

/// Generated by chat-gpt
impl<T> Debug for Matrix<T>
where T: Clone + Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Find the maximum string length of any element in the matrix
        let mut max_len = 0;
        for i in 0..self.rows_count {
            for j in 0..self.cols_count {
                let len = format!("{:?}", self[i][j]).len();
                if len > max_len {
                    max_len = len;
                }
            }
        }
        // Format each element with a fixed width
        for i in 0..self.rows_count {
            write!(f, "|")?;
            for j in 0..self.cols_count {
                let element = format!("{:width$?}", self[i][j], width = max_len);
                write!(f, " {} ", element)?;
            }
            write!(f, "|\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn matrix_from_rows() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 7.0),
        ])
        .unwrap();

        assert_eq!(m.get_row(0).unwrap(), Vector::from_xyz(1.0, 2.0, 3.0));
        assert_eq!(m.get_row(1).unwrap(), Vector::from_xyz(3.0, 4.0, 5.0));
        assert_eq!(m.get_row(2).unwrap(), Vector::from_xyz(5.0, 6.0, 7.0));
    }

    #[test]
    fn matrix_swap_cols() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 7.0),
        ])
        .unwrap();
       let mut m_clone = m.clone();
        m_clone.swap_cols(1, 2).unwrap();

        assert_eq!(m_clone.get_col(1), Some(Vector::from_xyz(3.0, 5.0, 7.0)));
        assert_eq!(m_clone.get_col(2), Some(Vector::from_xyz(2.0, 4.0, 6.0)));
    }

    #[test]
    fn matrix_determinant() {
        let m = Matrix::identity(4);
        assert_eq!(m.determinant().unwrap(), 1.0);
    }

    #[test]
    fn matrix_determinant_zeros() {
        let m = Matrix::zeroes(5, 5);
        assert_eq!(m.determinant().unwrap(), 0.0)
    }

    #[test]
    fn matrix_determinant_negative16() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 15.0),
        ])
       .unwrap();

        assert_eq!(m.determinant(), Some(-16.0));
    }

    #[test]
    fn matrix_determinant_non_square_matrix() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
           Vector::from_xyz(3.0, 4.0, 5.0),
        ])
        .unwrap();

        assert_eq!(m.determinant(), None);
    }

    #[test]
    fn matrix_determinant_empty_matrix() {
       let m = Matrix::empty();

        assert_eq!(m.determinant(), Some(1.0));
    }

    #[test]
    fn matrix_get_minor() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
           Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 15.0),
        ]).unwrap();
        
        let minor = Matrix::from_rows(vec![
            Vector::new(vec![1.0, 3.0]),
            Vector::new(vec![5.0, 15.0])
        ]).unwrap();

       let expected_minor = m.get_minor(vec![1], vec![1]);
        println!("{:?}", expected_minor);
        assert_eq!(minor, expected_minor);
    }

    #[test]
    fn matrix_inverse1() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 3.0, 7.0),
            Vector::from_xyz(2.0, 2.0, 1.0),
            Vector::from_xyz(3.0, 8.0, 6.0),
        ]).unwrap();
        
        assert!(m.determinant().unwrap().abs() > 1e-5);
        let inverse = m.inverse().unwrap();
        println!("inverse\n{:?}", inverse);
        let assert_identity = (&m * &inverse).unwrap();
        println!("identity\n{:?}", assert_identity);
        let identity = Matrix::identity(m.cols_count);
        assert!(assert_identity.approximately_equal(&identity, 1e-5));
   }

    #[test]
    fn matrix_inverse2() {
        let m = Matrix::from_rows(vec![
            Vector::new(vec![4.0, 3.0]),
            Vector::new(vec![3.0, 2.0]),
        ]).unwrap();

        let inv_expected = Matrix::from_rows(vec![
            Vector::new(vec![-2.0, 3.0]),
            Vector::new(vec![3.0, -4.0]),
        ]).unwrap();
          
        let inv = m.inverse().unwrap();

        assert_eq!(inv_expected, inv);
    }

    #[test]
    fn matrix_equal() {
        let a = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 15.0),
        ]).unwrap();
       
        let b = Matrix::from_cols(vec![
            Vector::from_xyz(1.0, 3.0, 5.0),
            Vector::from_xyz(2.0, 4.0, 6.0),
            Vector::from_xyz(3.0, 5.0, 15.0),
        ]).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_not_equal() {
        let a = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(3.0, 4.0, 5.0),
            Vector::from_xyz(5.0, 6.0, 15.0),
        ]).unwrap();
        let b = Matrix::zeroes(3, 4);
       
        assert_ne!(a, b);
    }

    #[test]
    fn matrix_equal2() {
        let a = Matrix::from_rule(2, 4, |i, j| (i * 4 + j) as f32);
        let b = a.clone();

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equal3() {
        let a = Matrix::from_rule(2, 4, |i, j| (i * 4 + j) as f32);
        let mut b = a.clone();
        b.set(1, 1, 100.0).unwrap();
        
        assert_ne!(a, b);
    }   
    
    #[test]
    fn matrix_adjoint1() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 2.0, 3.0),
            Vector::from_xyz(2.0, 3.0, 4.0),
            Vector::from_xyz(3.0, 4.0, 5.0)
        ]).unwrap();

        let adjoint_expected = Matrix::from_rows(vec![
            Vector::from_xyz(-1.0, 2.0, -1.0),
            Vector::from_xyz(2.0, -4.0, 2.0),
            Vector::from_xyz(-1.0, 2.0, -1.0)
        ]).unwrap().transposed();
        
        let adjoint_actual = m.adjoint_matrix().expect("bad test");

        assert!(adjoint_actual.approximately_equal(&adjoint_expected, 1e-5))
    }

    #[test]
    fn matrix_adjoint2() {
        let m = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 1.0, 1.0),
            Vector::from_xyz(1.0, 2.0, 1.0),
            Vector::from_xyz(3.0, 3.0, 2.0)
        ]).unwrap();

        let adjoint_expected = Matrix::from_rows(vec![
            Vector::from_xyz(1.0, 1.0, -1.0),
            Vector::from_xyz(1.0, -1.0, 0.0),
            Vector::from_xyz(-3.0, 0.0, 1.0)
        ]).unwrap();
        
        let adjoint_actual = m.adjoint_matrix().expect("bad test");
        print!("{:?}", adjoint_actual);
        assert!(adjoint_actual.approximately_equal(&adjoint_expected, 1e-5))
    }

    #[test]
    fn matrix_rotation_3d_halfpi_xyz() {
        let actual = Matrix::rotation_matrix3d(PI / 2.0, PI / 2.0, PI / 2.0);
        let rot_x = Matrix::new(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, 0.0, -1.0,
            0.0, 1.0, 0.0,
        ]).unwrap();
        let rot_y = Matrix::new(3, 3, vec![
            0.0, 0.0, 1.0,
            0.0, 1.0, 0.0,
            -1.0, 0.0, 0.0,
        ]).unwrap();
        let rot_z = Matrix::new(3, 3, vec![
            0.0, -1.0, 0.0,
            1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0,
        ]).unwrap();
        let expected = rot_x.multiply(&rot_y).unwrap().multiply(&rot_z).unwrap();
        println!("expected: \n{:?}\nactual: \n{:?}", expected, actual);
        assert!(actual.approximately_equal(&expected, 1e-5));
    }

    #[test]
    fn matrix_rotation_3d_halfpi_x() {
        let actual = Matrix::rotation_matrix3d(PI / 2.0, 0.0, 0.0);
        let rot_x = Matrix::new(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, 0.0, -1.0,
            0.0, 1.0, 0.0,
        ]).unwrap();
        let rot_y = Matrix::new(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]).unwrap();
        let rot_z = Matrix::new(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]).unwrap();
        let expected = rot_x.multiply(&rot_y).unwrap().multiply(&rot_z).unwrap();
        println!("expected: \n{:?}\nactual: \n{:?}", expected, actual);
        assert!(actual.approximately_equal(&expected, 1e-5));
    }
}