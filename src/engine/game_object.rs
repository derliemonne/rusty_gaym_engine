use crate::math::*;
use super::*;


pub trait GameObject {
    fn intersection_distance(ray: Ray) -> f32;
}