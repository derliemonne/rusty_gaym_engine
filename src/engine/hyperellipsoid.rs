use std::{ops::Mul, cmp::min};

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
        if ray.direction.dim() != ray.point.dim() || ray.direction.dim() != self.semiaxes.len() {
            return None
        }

        let n: usize = self.semiaxes.len();

        let a = Matrix::from_rule(n, n, |i, j| (
            if i != j {
                0.0
            }
            else {
                1.0 / (self.semiaxes[i] * self.semiaxes[i])
            }
        ));
        let r = Transform::default_direction().rotate_to_matrix3d(&self.transform.get_direction()).unwrap();
        let q = r.transposed().multiply(&a).unwrap().multiply(&r).unwrap();
        let l = Matrix::from_col(ray.direction);
        let p = Matrix::from_col((ray.point - self.transform.position.clone()).unwrap());

        let alpha = l.transposed().multiply(&q).unwrap().multiply(&l).unwrap()[0][0];
        let beta = l.transposed().multiply(&q).unwrap().multiply(&p).unwrap()[0][0];
        let gamma = p.transposed().multiply(&q).unwrap().multiply(&p).unwrap()[0][0];
        let discriminant = beta * beta - alpha * gamma;
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant_sqrt.is_nan() {
            return None;
        }

        let lambda1 = (-beta + discriminant_sqrt) / alpha;
        let lambda2 = (-beta - discriminant_sqrt) / alpha;

        let distance = f32::min(lambda1, lambda2);
        Some(distance)
    }
}