use crate::*;
use crate::components::TransformComponent;

pub trait Body {
    fn contains(&self, point: Vector) -> bool;
}

pub struct Sphere {
    transform: TransformComponent,
    radius: f32,
}


