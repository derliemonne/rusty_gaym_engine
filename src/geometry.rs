use crate::vector::Vector;


pub struct Ray {
    pub coordinate_system: CoordinateSystem,
    pub point: Vector,
    pub direction: Vector
}

#[derive(PartialEq)]
pub struct CoordinateSystem {
    pub initial_point: Vector,
    pub base: Vec<Vector>
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