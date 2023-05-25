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

#[derive(PartialEq, Clone)]
pub struct CoordinateSystem {
    pub initial_point: Vector<f32>,
    pub base: Vec<Vector<f32>>
}

impl CoordinateSystem {
    pub fn default3() -> CoordinateSystem {
        CoordinateSystem { 
            initial_point: Vector::zero3(),
            base: vec![Vector::from_xyz(1.0, 0.0, 0.0),
                       Vector::from_xyz(0.0, 1.0, 0.0),
                       Vector::from_xyz(0.0, 0.0, 1.0)]
        }
    }
}