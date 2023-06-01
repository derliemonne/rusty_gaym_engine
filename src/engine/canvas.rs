use super::*;
use crate::math::*;


struct Canvas {
    resolution: (usize, usize),
    distances: Matrix<f32>
}

impl Canvas {
    pub fn draw(&self) {
        println!("canvas.draw")
    }

    pub fn update(camera: &Camera, game_objects: &Vec<Box<dyn GameObject>>) {
        // update self.distances
    }
}