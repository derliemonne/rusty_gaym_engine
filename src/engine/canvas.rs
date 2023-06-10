use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::math::*;


pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub distances: Matrix<Option<f32>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { 
            width,
            height,
            distances: Matrix::from_rule(width, height, |_, _| None) 
        }
    }

    pub fn new_from_game_config(config: &GameConfig) -> Canvas {
        Canvas::new(config.screen_width, config.screen_height)
    }

    pub fn update<'a>(
    &mut self,
    camera: &Camera,
    game_objects: &GameObjects) {

        let rays = camera.get_rays_matrix(self.height, self.width);

        debug_assert_eq!(rays.cols_count(), self.width);
        debug_assert_eq!(rays.rows_count(), self.height);

        let min_distance = 
        |game_objects: &GameObjects, ray: &Ray| {
            let mut min_distance: Option<f32> = None;
            for game_object in game_objects {
                let distance = match game_object.borrow().intersection_distance(ray) {
                    None => continue,
                    Some(distance) => distance,
                };
                if min_distance.is_none() || distance < min_distance.unwrap() {
                    min_distance = Some(distance);
                }
            }
            min_distance
        };

        self.distances = Matrix::from_rule(
            self.height, self.width,
            |i, j| min_distance(game_objects, &rays[i][j])
        )
    }
}


#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn test() {
        // let mut canvas = Canvas::new(5, 5);
        // let camera = Camera::new(Transform::default(), &GameConfig::default());
        // let mut plane = Hyperplane { transform: Transform::new_from_coords(
        //     1.0, 0.0, 0.0,
        //     1.0, 1.0, 0.0,
        // ).unwrap()};
        // let mut game_objects = vec![];
        // game_objects.push(&mut plane as &mut dyn GameObject);

        // canvas.update(&camera, &game_objects);

        // println!("{:?}", canvas.distances);
    }
}