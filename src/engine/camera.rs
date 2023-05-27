use super::*;
use crate::math::*;


pub struct Camera {
    pub transform: Transform,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub draw_distance: f32,
}

impl Camera {
    pub fn get_rays_matrix(&self, n: usize, m: usize) -> Matrix<Ray> {
        // TODO: if fov >= pi than log warning message.

        let delta_alpha: f32 = self.horizontal_fov / n as f32;
        let delta_beta: f32 = self.vertical_fov / m as f32;
        let alpha_i = |i| delta_alpha * i as f32 - 0.5 * delta_alpha;
        let beta_j = |j| delta_beta * j as f32 - 0.5 * delta_beta;
        // View direction vector.
        let v: &Vector<f32> = &self.transform.rotation;
        let v_ij = |i, j| v.rotate3d(0.0, beta_j(j), alpha_i(i)).unwrap();
        // Fix "fish eye" effect.
        let v_fixed_ij = |i, j| {
            let v_ij = v_ij(i, j);
            v.square_magnitude() / v.dot_product(&v_ij) * &v_ij
        };

        Matrix::<Ray>::from_rule(
            n, m,
            |i, j| Ray::new(self.transform.position.clone(), v_fixed_ij(i, j))
        )
    }
}