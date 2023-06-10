use super::*;
use crate::math::*;

#[derive(Clone, Debug)]
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
    /// Creates transform from position and direction vectors.
    /// Those vectors must have dimenstion of 3.
    /// Direction vector will be normalized.
    pub fn new(position: Vector<f32>, direction: &Vector<f32>) -> Option<Transform> {
        if position.dim() != 3 || direction.dim() != 3 {
            return None;
        }
        
        let mut t = Transform::default();
        t.position = position;
        if t.set_direction(&direction).is_err() {
            return None;
        }
        Some(t)
    }

    /// Crates transform from position and direction vector components.
    /// Direction vector will be normalized.
    /// If direction vector is 
    pub fn new_from_coords(
    x: f32, y: f32, z: f32,
    direction_x: f32, direction_y: f32, direction_z: f32) -> Option<Transform> {
        let position = Vector::from_xyz(x, y, z);
        let direction = Vector::from_xyz(direction_x, direction_y, direction_z);
        Transform::new(position, &direction)
    }

    pub fn default_direction() -> Vector<f32> {
        Vector::from_xyz(1.0, 0.0, 0.0)
    }

    /// Returns normalized vector of direction.
    pub fn get_direction(&self) -> &Vector<f32> {
        debug_assert!((self.direction.square_magnitude() - 1.0).abs() < 1e-6);
        &self.direction
    } 

    /// Set direction vector. If passed vector is not normalized then it normalizes.
    /// If vector can not be normalized returns false.
    pub fn set_direction(&mut self, direction: &Vector<f32>) -> Result<(), ()> {
        self.direction = match direction.normalized() {
            NormalizedVectorResult::UnableToNormalize(v) => return Err(()),
            NormalizedVectorResult::Normalized(v) => v, 
        };
        return Ok(())
    }
}