pub mod math;
pub mod engine;
pub mod fov_utils;
use std::f32::consts::PI;
use math::*;
use engine::*;


fn main() {
    let config = GameConfig {
        screen_width: 20,
        screen_height: 20,
        target_fps: 30,
        camera_fov: PI / 3.0,
        camera_draw_distance: 20.0,
    };
    let mut game = Game::new(CoordinateSystem::default3(), config);
    game.start_loop();
}