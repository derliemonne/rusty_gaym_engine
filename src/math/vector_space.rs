use super::*;

pub struct VectorSpace {
    pub initial_point: Vector<f32>,
    pub basis_vectors: [Vector<f32>; 3],
}