use crate::math::*;
use super::*;

struct Hyperellipsoid {
    pub transform: Transform,
    pub semiaxes: Vec<f32>
}

impl Hyperellipsoid {
    pub fn planar_rotate(indices: (i32, i32), angle_radians: f32) {
        panic!()
    }
}

impl GameObject for Hyperellipsoid {
    /// https://math.stackexchange.com/questions/3309397/line-ellipsoid-intersection
    fn intersection_distance(&self, ray: Ray) -> Option<f32> {
        panic!();
        None
    }
}