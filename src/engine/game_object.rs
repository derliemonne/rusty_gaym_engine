use crate::math::*;
use super::*;


pub trait GameObject {
    fn intersection_distance(&self, transform: &Transform, ray: &Ray) -> Option<f32>;
}