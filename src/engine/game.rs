use super::*;
use crate::math::*;

use console_engine::ConsoleEngine;
use console_engine::Color;
use console_engine::KeyCode;
use console_engine::pixel::Pixel;



pub struct Game<'a> {
    pub coordinate_system: CoordinateSystem,
    pub entities: Vec<Entity>,
    pub systems: Vec<&'a dyn Fn(&Game) -> bool>,
    pub game_objects: Vec<Box<dyn GameObject>>,
    pub config: GameConfig,
    pub console_engine: ConsoleEngine,
    pub camera: Camera,
    pub canvas: Canvas,
}

impl<'a> Game<'a> {
    pub fn new(coordinate_system: CoordinateSystem, config: GameConfig) -> Game<'a> {
        Game { 
            coordinate_system, 
            entities: vec![],
            systems: vec![],
            game_objects: vec![],
            console_engine: ConsoleEngine::init(
                config.screen_width as u32,
                config.screen_height as u32,
                config.target_fps as u32
            ).expect("Internal problem with creating console engine."),
            camera: Camera::new_from_config(&config),
            canvas: Canvas::new_from_game_config(&config),
            config,
        }
    }

    pub fn start_loop(&mut self) {
        loop {
            let to_continue = self.update();
            if !to_continue {
                break;
            }
        }
    }

    pub fn create_entity(&mut self) -> &mut Entity {
        let entity = Entity::new();
        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }

    fn update(&mut self) -> bool {
        self.canvas.update(&self.camera, &self.game_objects);

        self.console_engine.wait_frame(); // wait for next frame + capture inputs
        self.console_engine.clear_screen();
    
        let (width, height) = self.canvas.resolution;
        let distances = &self.canvas.distances;
        let fg = |i, j| {
            match distances[i][j] {
                None => Color::Red,
                Some(dist) => {
                    let a: u8 = ((dist / self.config.camera_draw_distance) * 255.0) as u8;
                    Color::Rgb { r: a, g: a, b: a }
                }
            }
        };
        for i in 0..width {
            for j in 0..height {
                let char = 'O';
                let pixel = Pixel {
                    bg: Color::Black,
                    fg: fg(i, j),
                    chr: char,
                };
                self.console_engine.set_pxl(i as i32, j as i32, pixel);
            }
        }
        
        if self.console_engine.is_key_pressed(KeyCode::Char('q')) {
            return false;
        }
    
        self.console_engine.draw();


        for system in &self.systems {
            let shutdown = system(self);
            if shutdown {
                return false;
            }
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}

