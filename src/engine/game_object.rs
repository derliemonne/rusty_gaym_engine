use crate::math::*;
use super::*;


pub trait GameObject {
    fn intersection_distance(&self, ray: &Ray) -> Option<f32>;
}