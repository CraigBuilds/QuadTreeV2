use super::quad_tree::*;

#[derive(Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub collision: bool,
}

pub fn rebuild_tree(tree: &mut QuadTree<&mut Entity>, model: &mut Vec<Entity>) {
    tree.clear();
    for i in 0..model.len() {
        let entity = &mut model[i] as *mut Entity;
        let entity = unsafe {&mut *entity};
        //insert a reference to the entity into the tree
        tree.insert(entity.x, entity.y, entity);
    }
}

pub fn update_entity<'a>(entity: &mut Entity, local_model: &mut Vec<&'a mut Entity>) {
    for other_entity in local_model {
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