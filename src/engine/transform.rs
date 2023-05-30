use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct Transform {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform { position: Vector::zero3(), rotation: Transform::default_rotation() }
    }
}

impl Transform {
    pub fn default_rotation() -> Vector<f32> {
        Vector::from_xyz(1.0, 0.0, 0.0)
    }
}