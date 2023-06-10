pub mod math;
pub mod engine;
pub mod fov_utils;

use std::fmt::Debug;

use math::*;

fn print<T: Clone + Debug>(matrix: &Matrix<T>) {
    println!("{:?}", matrix);
}

fn main() {
    let a = Matrix::from_row(Vector::new(vec![1.0, 2.0, 3.0]));
    let b = Matrix::from_row(Vector::new(vec![1.0, 2.0, 4.0]));
    let o = Matrix::<f32>::empty();

    println!("{:?}", a.determinant());
    
    
}
