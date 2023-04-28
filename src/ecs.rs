use crate::geometry::*;
use crate::components::*;
use uuid::Uuid;
use std::collections::HashMap;

type DynComponent = Box<dyn Component>;

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
    pub components: Components<'a>,
    pub systems: Vec<&'a dyn Fn(&Game)>
}

impl<'a> Game<'a> {
    pub fn new(coordinate_system: CoordinateSystem) -> Game<'a> {
        Game { 
            coordinate_system, 
            entities: vec![],
            components: Components::default(),
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


#[derive(Default)]
pub struct Components<'a> {
    pub components: HashMap<Entity, Vec<&'a DynComponent>>,
}

impl<'a> Components<'a> {
    pub fn collect<T: Component + 'static>(&self) -> Vec<&T> {
        self.collect_dyn()
            .iter()
            .filter_map(|&c| c.as_any().downcast_ref::<T>())
            .collect()
    }

    pub fn collect_dyn(&self) -> Vec<&DynComponent> {
        self.components.iter()
        .flat_map(|(_entity, components)| components.clone())
        .collect()
    }

    pub fn with<T: Component + 'static>(&self) -> Components {   
        let good_components: HashMap<Entity, Vec<&DynComponent>> = 
            self.components.iter()
            .map(|(&entity, components)| (entity, self.filter::<T>(components)))
            .collect();

        Components {components: good_components}
    }

    pub fn add(&mut self, entity: Entity, component: &'a DynComponent) {
        let components = self.components.entry(entity).or_default();
        components.push(&component);
    }

    fn filter<T>(&self, components: &'a Vec<&'a DynComponent>) -> Vec<&'a DynComponent>
    where T: Component + 'static {
        components            
            .iter()
            .filter(|&c| c.as_any().downcast_ref::<T>().is_some())
            .map(|&c| c)
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components() {
        let mut game = Game::new(CoordinateSystem::default3());
        let e1 = Entity::new();
        let e2 = Entity::new();
        
        let c1: DynComponent = Box::new(TransformComponent::identity3d());
        let c2: DynComponent = Box::new(CameraComponent {draw_distance: 10.0, fov: 90.0});

        game.components.add(e1, &c1);
        game.components.add(e1, &c2);

        let tranform_components = game.components.with::<TransformComponent>();
        let transforms = tranform_components.collect::<TransformComponent>();
        assert_eq!(transforms.len(), 1)
    }
}

