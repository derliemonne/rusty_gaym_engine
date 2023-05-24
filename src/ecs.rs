use crate::geometry::*;
use uuid::Uuid;
use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: Uuid,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { id: Uuid::new_v4() }
    }
}

pub struct Game<'a> {
    pub coordinate_system: CoordinateSystem,
    pub entities: Vec<Entity>,
    pub systems: Vec<&'a dyn Fn(&Game)>
}

impl<'a> Game<'a> {
    pub fn new(coordinate_system: CoordinateSystem) -> Game<'a> {
        Game { 
            coordinate_system, 
            entities: vec![],
            systems: vec![],
        }
    }

    pub fn start_loop(&self) -> ! {
        loop {
            self.update();
        }
    }

    pub fn create_entity(&mut self) -> &mut Entity {
        let entity = Entity::new();
        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }

    fn update(&self) {
        for system in &self.systems {
            system(self);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}

