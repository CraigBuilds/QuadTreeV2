use super::quad_tree::{GetX, GetY};

#[derive(Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub collision: bool,
}

impl GetX for Entity {
    fn get_x(&self) -> u16 {
        self.x
    }
}

impl GetY for Entity {
    fn get_y(&self) -> u16 {
        self.y
    }
}

pub fn update_entity_local<'a>(entity: &mut Entity, local_model: &mut Vec<&'a mut Entity>) {
    for other_entity in local_model {
        if is_coliding(entity, other_entity) {
            entity.collision = true;
        }
    }
}

#[allow(dead_code)]
pub fn update_entity_global<'a>(entity: &mut Entity, model: &mut Vec<Entity>) {
    for other_entity in model {
        if is_coliding(entity, other_entity) {
            entity.collision = true;
        }
    }
}

pub fn is_coliding(entity: &Entity, other_entity: &Entity) -> bool {
    entity.x < other_entity.x + other_entity.width &&
    entity.x + entity.width > other_entity.x &&
    entity.y < other_entity.y + other_entity.height &&
    entity.y + entity.height > other_entity.y
}