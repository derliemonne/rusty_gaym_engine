pub mod math;

use math::*;
use vector::*;

fn print(matrix: &Matrix) {
    println!("{:#?}", matrix);
}

fn main() {
    let a = Matrix::from_row(&Vector::new(vec![1.0, 2.0, 3.0]));
    let b = Matrix::from_row(&Vector::new(vec![1.0, 2.0, 4.0]));
    let o = Matrix::empty();

    println!("{:?}", a.determinant());
    
    
}
