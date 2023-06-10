use super::*;
use crate::math::*;
use std::cell::RefCell;
use std::default;
use std::rc::Rc;
use std::time;
use std::time::Instant;

use console_engine::ConsoleEngine;
use console_engine::Color;
use console_engine::KeyCode;
use console_engine::pixel::Pixel;


pub type GameObjects = Vec<Rc<RefCell<dyn GameObject>>>;



#[derive(Default)]
pub struct MovingPlane(Hyperplane);

impl MovingPlane  {
    pub fn update<E, Es>(&mut self, game: &Game<E, Es>) where
    E: EventT + 'static,
    Es: EventSystemT<E> + 'static, {
        let t: f32 = match &game.clock {
            Clock::Inactive => panic!("clock is not working"),
            Clock::Active(active_clock) => active_clock.elapsed_from_start(),
        }.as_secs_f32();
        // self.0.transform.position[0] = t.sin() * 5.0;
        // self.0.transform.set_direction(&Vector::from_xyz(t, 0.0, 0.0)).unwrap();
    }
}

pub struct ActiveClock {
    started: time::Instant,
    prev_frame: time::Instant,
    current_frame: time::Instant,
}

impl ActiveClock {
    pub fn delta(&self) -> time::Duration {
        self.current_frame - self.prev_frame
    }

    pub fn elapsed_from_start(&self) -> time::Duration {
        self.started.elapsed()
    }
}

#[derive(Default)]
pub enum Clock {
    #[default]
    Inactive,
    Active(ActiveClock),
}

pub struct Game<'a, E, Es> where 
E: EventT + 'static,
Es: EventSystemT<E> + 'static, {
    pub coordinate_system: CoordinateSystem,
    pub entities: Vec<Entity>,
    pub config: GameConfig,
    pub console_engine: ConsoleEngine,
    pub camera: Camera,
    pub canvas: Canvas,
    pub event_system: Es,
    pub clock: Clock,
    pub moving_plane: RefCell<MovingPlane>,
    pub game_objects: GameObjects,
    _phantom_data: Option<&'a E>,
}

impl<'a, E, Es> Game<'a, E, Es> where
E: EventT + 'static,
Es: EventSystemT<E> + 'static {
    pub fn new(coordinate_system: CoordinateSystem, config: GameConfig) -> Game<'a, E, Es> {
        Game { 
            coordinate_system, 
            entities: vec![],
            console_engine: ConsoleEngine::init(
                config.screen_width as u32,
                config.screen_height as u32,
                config.target_fps as u32
            ).expect("Internal problem with creating console engine."),
            camera: Camera::new(Transform::default(), &config),
            canvas: Canvas::new_from_game_config(&config),
            config,
            event_system: Es::default(),
            clock: Clock::default(),
            moving_plane: RefCell::new(MovingPlane::default()),
            game_objects: vec![],
            _phantom_data: None,
        }
    }

    // Call before start_loop
    pub fn init(&mut self) {
        self.moving_plane = RefCell::new(MovingPlane(Hyperplane { transform: Transform::new_from_coords(
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
        ).unwrap()}));
    }

    // Call after init
    pub fn start_loop(&mut self) {
        let now = Instant::now();
        self.clock = Clock::Active(ActiveClock {
            started: now,
            prev_frame: now,
            current_frame: now,
        });

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
        // update of game objects state
        self.moving_plane.borrow_mut().update(self);
        // ----------------------------

        self.canvas.update(&self.camera, &self.game_objects);

        self.console_engine.wait_frame(); // wait for next frame + capture inputs
        if let Clock::Active(active_clock) = &mut self.clock {
            active_clock.prev_frame = active_clock.current_frame;
            active_clock.current_frame = Instant::now();
        }

        self.console_engine.clear_screen();
    
        let width = self.canvas.width;
        let height = self.canvas.height;
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
        for i in 0..height {
            for j in 0..width {
                let char = 'O';
                let char = match distances[i][j] {
                    None => 'n',
                    Some(d) => format!("{}", d).chars().nth(0).unwrap(),
                };
                let pixel = Pixel {
                    bg: Color::Black,
                    fg: fg(i, j),
                    chr: char,
                };
                self.console_engine.set_pxl(i as i32, j as i32, pixel);
            }
        }

        let debug_clock_text: String = match &self.clock {
            Clock::Inactive => 
                String::from("inactive clock"),
            Clock::Active(active_clock) =>
                format!(
                    "Secs from start: {}, delta_secs: {} ",
                    active_clock.started.elapsed().as_secs_f32(),
                    active_clock.delta().as_secs_f32(),
                ),
            
        };
        let debug_plane_text = format!("Plane: {:?}", self.moving_plane.borrow().0.transform.position);
        let debug_text = debug_clock_text.clone() + " " + &debug_plane_text;
        // Debug draw:
        self.console_engine.print_fbg(
            0,
            height as i32 - 1,
            &debug_clock_text,
            Color::Black,
            Color::White
        );
        self.console_engine.print_fbg(
            0,
            height as i32 - 2,
            &debug_plane_text,
            Color::Black,
            Color::White
        );
    
        
        if self.console_engine.is_key_pressed(KeyCode::Char('q')) {
            return false;
        }
    
        self.console_engine.draw();

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}

