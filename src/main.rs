pub mod matrix;
pub mod vector;
pub mod geometry;
pub mod engine;
pub mod components;
pub mod ecs;
use std::{vec};

use matrix::*;
use vector::*;
use geometry::*;
use ecs::*;
use components::*;


fn main() {
    let coordinate_system = CoordinateSystem::default3();
    let mut game = Game::new(coordinate_system);
    
    game.systems.push(&spam);
    game.start_loop();

}

fn spam(game: &Game) {
    println!("Spam system!");
}
