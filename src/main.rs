pub mod math;
pub mod engine;

use std::{vec};
use math::*;
use engine::*;


fn main() {
    let coordinate_system = CoordinateSystem::default3();
    let mut game = Game::new(coordinate_system);
    
    game.systems.push(&spam);
    game.start_loop();
}

fn spam(game: &Game) {
    println!("Spam system!");
}
