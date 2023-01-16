mod quad_tree;
use quad_tree::*;

#[derive(Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub collision: bool,
}

fn main() {
    
    let mut model = vec![
        Entity{x: 0, y: 0, width: 2, height: 2, collision: false},
        Entity{x: 1, y: 1, width: 2, height: 2, collision: false},
        Entity{x: 20, y: 20, width: 2, height: 2, collision: false},
        Entity{x: 21, y: 21, width: 2, height: 2, collision: false},
    ];

    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    '_main: loop {

        //rebuild the tree
        tree.clear();
        for i in 0..model.len() {
            let entity = &mut model[i] as *mut Entity;
            let entity = unsafe {&mut *entity};
            //insert a reference to the entity into the tree
            tree.insert(entity.x, entity.y, entity);
        }

        for entity in model.iter_mut() {
            let local_model = &mut tree.get_mut_leaf_around(entity.x, entity.y).unwrap().data;
            update_entity(entity, local_model);
        }
    }
}

fn update_entity<'a>(entity: &mut Entity, local_model: &mut Vec<&'a mut Entity>) {
    for other_entity in local_model {
        if is_coliding(entity, other_entity) {
            entity.collision = true;
        }
    }
}

fn is_coliding(entity: &Entity, other_entity: &Entity) -> bool {
    entity.x < other_entity.x + other_entity.width &&
    entity.x + entity.width > other_entity.x &&
    entity.y < other_entity.y + other_entity.height &&
    entity.y + entity.height > other_entity.y
}