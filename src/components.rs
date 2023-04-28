use std::{any::Any, fmt::Debug, io::ErrorKind};

use crate::*;


pub trait Component : Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct TransformComponent {
    pub position: Vector,
    pub rotation: Vector,
}

impl Component for TransformComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TransformComponent {
    pub fn identity3d() -> TransformComponent {
        TransformComponent {
            position: Vector::zero3(),
            rotation: Vector::zero3()
        }
    }

    pub fn rotate3d(&mut self, axis: Vector, x_radians: f32, y_radians: f32, z_radians: f32) {
        self.rotation.rotate3d(x_radians, y_radians, z_radians);
    }

    pub fn look_at3d(&mut self, object: Vector, view_vector: Vector) {
        if object.dim() != 3 || view_vector.dim() != 3 {
            panic!("Provided vectors are not 3d.")
        }
        self.position = (object - view_vector.clone()).unwrap();
        self.rotation = view_vector;
    }
}

#[derive(Debug)]
pub struct CameraComponent {
    pub fov: f32,
    pub draw_distance: f32,
}

impl Component for CameraComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}