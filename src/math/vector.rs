use std::iter::zip;
use std::ops::{MulAssign, Mul};
use std::{ops, vec};
use super::*;


#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub elements: Vec<T>,
}

impl<T> Vector<T> {
    pub const fn new(coordinates: Vec<T>) -> Vector<T> {
        Vector { elements: coordinates }
    }

    /// Returns number of components in vector.
    pub fn dim(&self) -> usize {
        self.elements.len()
    }

    /// Access vector component.
    pub fn get(&self, index: usize) -> Option<f32> {
        match self.elements.get(index) {
            None => return None,
            Some(x) => return Some(*x),
        }
    }

    pub fn set(&mut self, index: usize, value: f32) -> Result<(), ()> {
        match self.elements.get_mut(index) {
            None => return Err(()),
            Some(x) => *x = value,
        };
        Ok(())
    }
}

impl Vector<f32> {
    pub fn zero3() -> Vector<f32> {
        Vector::from_xyz(0.0, 0.0, 0.0)
    }

    pub fn one3() -> Vector<f32> {
        Vector::from_xyz(1.0, 1.0, 1.0)
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Vector<f32> {
        Vector::new(vec![x, y, z])
    }

    /// Rotates 2d vector counterclockwise on angle expressed in radians.
    /// Returns rotated 2d vector.
    /// If provided vector is not 2d, returns `None`.
    pub fn rotate2d(&self, radians: f32) -> Option<Vector<f32>> {
        if self.dim() != 2 {
            return None;
        }
        Some(Matrix::rotation_matrix2d(radians)
            .multiply_by_vector(self)
            .unwrap())
    }

    /// Rotates a vector along each of the three axes by the given angles for each axis.
    /// Returns rotated 3d vector.
    /// If provided vector is not 3d, returns `None`.
    pub fn rotate3d(&self, x_radians: f32, y_radians: f32, z_radians: f32) -> Option<Vector<f32>> {
        if self.dim() != 3 {
            return None;
        }
        Some(Matrix::rotation_matrix3d(x_radians, y_radians, z_radians)
            .multiply_by_vector(self)
            .unwrap())
    }

    pub fn normalized(&self) -> Vector<f32> {
        self / self.magnitude()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    /// Returns the square of Eucledean distance between two vectors.
    pub fn square_distance(&self, coordinates: &Vector<f32>) -> f32 {
        zip(self.elements.iter(), coordinates.elements.iter())
            .map(|(x1, x2)| (x2 - x1) * (x2 - x1))
            .sum()
    }

    /// Returns the Euclidean distance between two vectors.
    /// This method first calculates the square of the distance
    /// between the two vectors, and then takes the square root of the result.
    /// If you want to get square distance use `square_distance` method instead.
    pub fn distance(&self, b: &Vector<f32>) -> f32 {
        self.square_distance(b).sqrt()
        
    }

    /// Returns the square of Euclidean magnitude of vector.
    pub fn square_magnitude(&self) -> f32 {
        self.square_distance(&Vector::zero3())
    }

    /// Returns Euclidean magnitude of vector.
    /// This method first calculates the square magnitude, and then takes the square root of result.
    /// If you want to get square magnitude use `square_magnitude` method instead.
    pub fn magnitude(&self) -> f32 {
        self.distance(&Vector::zero3())
    }

    /// Computes the dot product of this vector with another vector.
    ///
    /// The dot product of two vectors is defined as the sum of the products of their corresponding
    /// coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    /// let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
    /// assert_eq!(v1.dot_product(&v2), 32.0);
    /// ```
    pub fn dot_product(&self, other: &Vector<f32>) -> f32 {
        zip(self.elements.clone(), other.elements.clone())
            .map(|(x1, x2)| x1 * x2)
            .sum()
    }

    /// Returns cross product of 3d vector and `other` 3d vector.
    /// If vectors are not 3d returns `None`.
    pub fn cross_product(&self, other: &Vector<f32>) -> Option<Vector<f32>> {
        if self.dim() != 3 || other.dim() != 3 {
            return None;
        }
        Some(Vector::from_xyz(
            self[1] * other[2] - other[1] * self[2],
            other[0] * self[2] - self[0] * other[2],
            self[0] * other[1] - other[0] * self[1]))
    }

    /// Vectors are approximately equal if the absolute difference between corresponding vector elements are less than `epsilon`.
    pub fn approximately_equal(&self, other: &Vector<f32>, epsilon: f32) -> bool {
        zip(self.elements.iter(), other.elements.iter())
            .all(|(x1, x2)| (x1 - x2).abs() < epsilon)
    }
}

impl<T> ops::Index<usize> for Vector<T> {
    type Output = f32;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v = Vector::new(vec![1.0, 5.6, 9.0, -0.1]);
    /// assert_eq!(v[3], -0.1);
    /// ```
    /// Next line of code will panic.
    /// ```should_panic
    /// # use rusty_gaym_engine::vector::Vector;
    /// # let v = Vector::new(vec![1.0, 5.6, 9.0, -0.1]);
    /// v[4];
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
        
    }
}

impl<T> ops::IndexMut<usize> for Vector<T> {
    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let mut v = Vector::new(vec![1.0, 3.9, 0.0, 1.4]);
    /// v[2] = 10.0;
    /// assert_eq!(v, Vector::new(vec![1.0, 3.9, 10.0, 1.4]));
    /// ```
    /// Next line will cause program to panic.
    /// ```should_panic
    /// # use rusty_gaym_engine::vector::Vector;
    /// # let mut v = Vector::new(vec![1.0, 3.9, 0.0, 1.4]);
    /// v[4] = 1.0;
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.elements.get_mut(index).unwrap()
    }
}

impl<T> ops::Add<T> for Vector<T> {
    type Output = Option<Vector<T>>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// let v2 = Vector::from_xyz(4.0, -2.0, 10.0);
    /// assert_eq!(v1 + v2, Some(Vector::from_xyz(5.0, 0.0, 13.0)));
    /// ```
    /// If vectors\` dimensions are different the sum will return `None`:
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    /// let v2 = Vector::new(vec![4.0, -2.0, 10.0, 13.3]);
    /// assert_eq!(v1 + v2, None);
    fn add(self, rhs: Vector<f32>) -> Self::Output {
        if self.dim() != rhs.dim() {
            return None;
        }

        Some(Vector::new(
            zip(self.elements, rhs.elements)
                .map(|(self_x, rhs_x)| self_x + rhs_x)
                .collect(),
        ))
    }
}

impl ops::Sub<Vector<f32>> for Vector<f32> {
    type Output = Option<Vector<f32>>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// let v2 = Vector::from_xyz(4.0, -2.0, 10.0);
    /// assert_eq!(v1 - v2, Some(Vector::from_xyz(-3.0, 4.0, -7.0)));
    /// ```
    /// If vectors\` dimensions are different the sum will return `None`:
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    /// let v2 = Vector::new(vec![4.0, -2.0, 10.0, 13.3]);
    /// assert_eq!(v1 - v2, None);
    fn sub(self, rhs: Vector<f32>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T> ops::Neg for Vector<T> {
    type Output = Vector<f32>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// assert_eq!(-v1, Vector::from_xyz(-1.0, -2.0, -3.0));
    /// ```
    fn neg(self) -> Self::Output {
        -1.0 * &self
    }
}

impl<T> ops::Mul<T> for &Vector<T> {
    type Output = Vector<T>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// assert_eq!(v1 * 2.0, Vector::from_xyz(2.0, 4.0, 6.0));
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            elements: self.elements.iter().map(|x| x * rhs).collect(),
        }
    }
}

impl<T> ops::MulAssign<T> for Vector<T> {
    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let mut v = Vector::from_xyz(1.0, 2.0, 3.0);
    /// v *= 5.0;
    /// assert_eq!(v, Vector::from_xyz(5.0, 10.0, 15.0));
    /// ```
    fn mul_assign(&mut self, rhs: T) {
        self.elements.iter_mut().for_each(|c| *c *= rhs)
    }
}

impl ops::Mul<&Vector<f32>> for f32 {
    type Output = Vector<f32>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// assert_eq!(2.0 * v1, Vector::from_xyz(2.0, 4.0, 6.0));
    /// ```
    fn mul(self, rhs: &Vector<f32>) -> Self::Output {
        rhs * self
    }
}

impl<T> ops::Div<T> for &Vector<T> {
    type Output = Vector<T>;

    /// # Example
    /// ```
    /// # use rusty_gaym_engine::vector::Vector;
    /// let v1 = Vector::from_xyz(1.0, 2.0, 3.0);
    /// assert_eq!(&v1 / 10.0, Vector::from_xyz(0.1, 0.2, 0.3));
    /// assert_eq!(&v1 / 0.0, Vector::from_xyz(f32::INFINITY, f32::INFINITY, f32::INFINITY));
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<T> ops::DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        *self *= 1.0 / rhs
    }
}

impl<T> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn compare_vectors(
        actual: &Vector<f32>,
        expected: &Vector<f32>,
        approximately_equal: bool,
        epsilon: Option<f32>,
    ) {
        let vectors_approximately_equal =
            zip(actual.elements.iter(), expected.elements.iter()).all(
                |(x_actual, x_expected)| (x_actual - x_expected).abs() < epsilon.unwrap_or(1e-5),
            );

        assert!(
            vectors_approximately_equal == approximately_equal,
            "expected {:?}, got: {:?}",
            expected,
            actual
        );
    }

    #[test]
    fn approximately_equal() {
        let v1 = Vector::from_xyz(1.0, 5.5, 4.3);
        let v2 = Vector::from_xyz(1.0 + f32::EPSILON, 5.5 - f32::EPSILON, 4.3);
        assert!(v1.approximately_equal(&v2, f32::EPSILON * 2.0));
    }

    #[test]
    fn approximately_not_equal() {
        let v1 = Vector::from_xyz(0.0, 1.3, -42.3);
        let v2 = Vector::from_xyz(0.0, 1.3, -42.3 + 0.1);

        assert!(!v1.approximately_equal(&v2, 0.01));
    }

    #[test]
    fn add_vectors() {
        let v1 = Vector::from_xyz(1.0, 5.5, 4.3);
        let v2 = Vector::from_xyz(2.0, 5.9, -1.0);
        let actual = (v1 + v2).unwrap();
        let expected = Vector::from_xyz(3.0, 11.4, 3.3);

        compare_vectors(&actual, &expected, true, None);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Vector::from_xyz(4.4, 10.3, 5.4);
        let v2 = Vector::from_xyz(0.0, -12.0, 4.5);
        let actual = (v1 - v2).unwrap();
        let expected = Vector::from_xyz(4.4, 22.3, 0.9);

        compare_vectors(&actual, &expected, true, None);
    }
    
    #[test]
    fn vector_equal1() {
        let a = Vector::from_xyz(1.0, 2.0, 3.0);
        let b = Vector::from_xyz(1.0, 2.0, 3.0);

        assert_eq!(a, b);
    }

    #[test]
    fn vector_equal2() {
        let a = Vector::from_xyz(1.0, 2.0, 3.0);
       let b = a.clone();

        assert_eq!(a, b);
    }

    fn vector_rotate2d(x: f32, y: f32, radians: f32, expected_x: f32, expected_y: f32) {
        let v = Vector::new(vec![x, y]);
        let expected = Vector::new(vec![expected_x, expected_y]);
        let actual = v.rotate2d(radians).unwrap();
        
        assert!(
            actual.approximately_equal(&expected, 1e-5),
            "expected: {:?}, actual: {:?}", expected, actual
        );
    }

    #[test]
    fn vector_rotate2d_1() {
        vector_rotate2d(1.0, 0.0, 90_f32.to_radians(), 0.0, 1.0);
    }

    #[test]
    fn vector_rotate2d_2() {
        vector_rotate2d(1.0, 0.0, 180_f32.to_radians(), -1.0, 0.0);
    }

    #[test]
    fn vector_rotate2d_3() {
        vector_rotate2d(1.0, 0.0, 270_f32.to_radians(), 0.0, -1.0);
    }

    #[test]
    fn vector_rotate2d_4() {
        vector_rotate2d(1.0, 0.0, 360_f32.to_radians(), 1.0, 0.0);
    }
}