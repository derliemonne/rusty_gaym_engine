pub mod math;
pub mod engine;
pub mod fov_utils;
use std::f32::consts::PI;
use math::*;
use engine::*;


#[derive(Debug)]
pub enum Event {

}

impl EventT for Event {}



#[derive(Default, Debug)]
pub struct EventSystem {
    pub events: Vec<Event>,
}

impl EventSystemT<Event> for EventSystem {
    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn on_trigger(&mut self, event: &Event) {
        dbg!("ON EVENT TRIGGER!");
    }
}



fn main() {
    let config = GameConfig {
        screen_width: 60,
        screen_height: 20,
        target_fps: 30,
        camera_fov: PI / 3.0,
        camera_draw_distance: 0.0,
    };
    let mut game = Game::<Event, EventSystem>::new(
        CoordinateSystem::default3(),
        config
    );    

    game.init();
    game.start_loop();
}