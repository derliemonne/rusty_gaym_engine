use super::*;
use crate::math::*;


struct Hyperplane {
    pub transform: Transform,
}

impl GameObject for Hyperplane {
    /// If dimensions of `ray` and `self.transform` are not the same returns `None`.
    /// If ray is inside the hyperplane the distance is `0`.
    /// If ray is parallel to hyperplane returns `None`.
    fn intersection_distance(&self, ray: Ray) -> Option<f32> {
        if ray.direction.dim() != self.transform.get_direction().dim() ||
            self.transform.get_direction().dim() != self.transform.position.dim() {
            return None;
        }
        
        // Let hyperplane alpha be a1 * x1 + a2 * x2 + ... + an * xn = b
        // Vector (a1, a2, ..., an) is normal to hyperplane and stored in the transform.rotation.
        let normal = &self.transform.get_direction();
        // A point on the hyperplane is stored in transform.position.
        let b = normal.dot_product(&self.transform.position);


        let normal_dot_ray_direction = normal.dot_product(&ray.direction);
        let normal_dot_ray_point = normal.dot_product(&ray.point);
        // If ray direction is collinear to hyperplane.
        if normal_dot_ray_direction == 0.0 {
            // If point of ray is inside hyperplane than the distance is 0.
            if normal_dot_ray_point == b {
                return Some(0.0);
            }
            // Ray is parallel to hyperplane. Ray and hyperplane never intersect.
            else {
                return None;
            }
        }

        // Line equation: X = ray.point + t * ray.direction. (1)
        // Where t is parameter: direction from ray.point to point X on line.
        // Plane equation: normal * X = b. (2)
        // We want to find points that satisfy both equations: (1) and (2).
        // Thats how we find t.
        let t = (b - normal_dot_ray_point) / normal_dot_ray_direction;
        Some(t)
    }
}