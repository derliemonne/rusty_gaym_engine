use super::*;
use crate::math::*;


pub struct Canvas {
    pub resolution: (usize, usize),
    pub distances: Matrix<Option<f32>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { 
            resolution: (width, height),
            distances: Matrix::from_rule(width, height, |_, _| None) 
        }
    }

    pub fn new_from_game_config(config: &GameConfig) -> Canvas {
        Canvas::new(config.screen_width, config.screen_height)
    }

    pub fn draw(&self) {
        println!("canvas.draw")
    }

    pub fn update(&mut self, camera: &Camera, game_objects: &Vec<Box<dyn GameObject>>) {
        let rays = camera.get_rays_matrix(self.resolution.0, self.resolution.1);

        debug_assert_eq!(rays.cols_count(), self.resolution.0);
        debug_assert_eq!(rays.rows_count(), self.resolution.1);

        let min_distance = |game_objects: &Vec<Box<dyn GameObject>>, ray: &Ray| {
            let mut min_distance: Option<f32> = None;
            for game_object in game_objects {
                let distance = match game_object.intersection_distance(ray) {
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
            self.resolution.0, self.resolution.1,
            |i, j| min_distance(game_objects, &rays[i][j])
        )
    }
}