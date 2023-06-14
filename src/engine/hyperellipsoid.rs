use crate::math::*;
use super::*;
use crate::utils::assert_eq_f32;

#[derive(Debug, Clone)]
pub struct Hyperellipsoid {
    pub semiaxes: Vec<f32>
}

impl Hyperellipsoid {
    pub fn new3d(a: f32, b: f32, c: f32) -> Self {
        Self{ semiaxes: vec![a, b, c] }
    }
}

impl GameObject for Hyperellipsoid {
    /// https://math.stackexchange.com/questions/3309397/line-ellipsoid-intersection
    fn intersection_distance(&self, transform: &Transform, ray: &Ray) -> Option<f32> {
        if ray.direction.dim() != 3 || ray.point.dim() != 3 || ray.direction.dim() != 3 || self.semiaxes.len() != 3 {
            panic!("Not 3d");
        }

        let (a, b, c) = (self.semiaxes[0], self.semiaxes[1], self.semiaxes[2]);
        let x0 = (ray.point.clone() - (Vector::zero3() - transform.position.clone()).unwrap()).unwrap();
        let (x0, y0, z0) = (x0[0], x0[1], x0[2]);

        let (alpha, beta, gamma) = (ray.direction[0], ray.direction[1], ray.direction[2]);

        let lambda1 = 
            alpha * alpha * b * b * c * c +
            beta * beta * a * a * c * c +
            gamma * gamma * a * a * b * b; 
        let lambda2 = 
            (x0 * alpha * b * b * c * c) +
            (y0 * beta * a * a * c * c) + 
            (z0 * gamma * a * a * b * b);
        let lambda3 =
            x0 * x0 * b * b * c * c +
            y0 * y0 * a * a * c * c +
            z0 * z0 * a * a * b * b -
            a * a * b * b * c * c;
        let discriminant = lambda2 * lambda2 - lambda1 * lambda3;
        if discriminant < 0.0 {
            return None;
        }
        let dist1 = (-lambda2 + discriminant.sqrt()) / lambda1;
        let dist2 = (-lambda2 - discriminant.sqrt()) / lambda1;
        if dist1 < 0.0 && dist2 < 0.0 {
            return None;
        }
        if dist1 < 0.0 {
            return Some(dist2);
        }
        if dist2 < 0.0 {
            return Some(dist1);
        }
        return Some(f32::min(dist1, dist2));
    }
}



#[cfg(test)]
mod hyperellipsoid_tests {
    use crate::utils::assert_eq_option_f32;

    use super::*;

    fn test(
        a: f32, b: f32, c: f32,
        x0: f32, y0: f32, z0: f32,
        alpha0: f32, beta0: f32, gamma0: f32,
        alpha: f32, beta: f32, gamma: f32,
        expected_distance: Option<f32>,
    ) {
        let e = Hyperellipsoid::new3d(a, b, c);
        let t = Transform::new_from_coords(
            x0, y0, z0,
            1.0, 0.0, 0.0,
        ).unwrap();
        let r = Ray::new(Vector::from_xyz(alpha0, beta0, gamma0), Vector::from_xyz(alpha, beta, gamma));
        assert_eq_option_f32(e.intersection_distance(&t, &r), expected_distance);
    }

    #[test]
    fn test1() {
        test(
            1.0, 1.0, 1.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            Some(1.0),
        )
    }

    #[test]
    fn test2() {
        test(
            1.0, 1.0, 1.0,
            1.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            Some(0.0),
        )
    }
}