use uuid::Uuid;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: Uuid,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { id: Uuid::new_v4() }
    }
}