pub mod matrix;
pub mod vector;
mod game;
use matrix::*;
use vector::*;


fn main() {
    let matrix = Matrix::from_rule(3, 4, |i, j| (i + j) as f32);
    
    assert_eq!(matrix, Matrix::from_rows(vec![
        Vector::new(vec![0.0, 1.0, 2.0, 3.0]),
        Vector::new(vec![1.0, 2.0, 3.0, 4.0]),
        Vector::new(vec![2.0, 3.0, 4.0, 5.0])
    ]).unwrap());
    
}
