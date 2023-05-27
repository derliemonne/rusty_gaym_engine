use super::*;
use crate::math::*;

#[derive(Clone)]
pub struct Transform {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
}