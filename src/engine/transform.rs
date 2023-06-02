use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct Transform {
    pub position: Vector<f32>,
    direction: Vector<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform { position: Vector::zero3(), direction: Transform::default_direction() }
    }
}

impl Transform {
    pub fn default_direction() -> Vector<f32> {
        Vector::from_xyz(1.0, 0.0, 0.0)
    }

    /// Returns normalized vector of direction.
    pub fn get_direction(&self) -> &Vector<f32> {
        debug_assert!((self.direction.square_magnitude() - 1.0).abs() < 1e-6);
        &self.direction
    } 

    /// Set direction vector. If passed vector is not normalized then it normalizes.
    pub fn set_direction(&mut self, direction: &Vector<f32>) {
        self.direction = direction.normalized();
    }
}