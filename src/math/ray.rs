use super::*;


#[derive(Clone)]
pub struct Ray {
    pub coordinate_system: CoordinateSystem,
    pub point: Vector<f32>,
    pub direction: Vector<f32>
}

impl Ray {
    pub fn normalized(&self) -> Ray {
        let mut ray = self.clone();
        ray.direction.normalize();
        ray
    }

    pub fn normalize(&mut self) {
        self.direction.normalize();
    }
}