use crate::Vector;

pub trait Body {
    fn contains(&self, point: Vector) -> bool;
}

pub struct Transform {
    position: Vector,
    rotation: Vector,
}

pub struct Sphere {
    transform: Transform,
    radius: f32,
}

pub struct Camera {
    pub position: Vector,
    pub look_direction: Vector,
    pub fov: f32,
    pub draw_distance: f32,
}