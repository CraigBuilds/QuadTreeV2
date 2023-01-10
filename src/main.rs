mod quad_tree;
use quad_tree::*;

#[derive(Debug, Clone)]
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
    ];

    loop {

        //rebuild the tree
        let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16
        for entity in model.iter() {
            //insert a reference to the entity into the tree
            tree.insert(entity.x, entity.y, entity);
        }

        //broad phase
        let mut checks = vec![];
        for index in 0..model.len() {
            let entity = &model[index];
            let leaf = tree.get_leaf_around(entity.x, entity.y).unwrap();
            for (_, _, other_entity) in leaf.vec.iter() {
                //get position of other_entity within the model
                let other_index = model.iter().position(|e| e as *const Entity == *other_entity as *const Entity).unwrap();
                checks.push((index, other_index));
            }
        }

        //narrow phase
        for (index, other_index) in checks {
            let entity = &mut model[index].clone();
            let other_entity = &mut model[other_index];
            if is_coliding(entity, other_entity) {
                other_entity.collision = true;
            }
            else {
                other_entity.collision = false;
            }
        }
    }
}

fn is_coliding(entity: &Entity, other_entity: &Entity) -> bool {
    entity.x < other_entity.x + other_entity.width &&
    entity.x + entity.width > other_entity.x &&
    entity.y < other_entity.y + other_entity.height &&
    entity.y + entity.height > other_entity.y
}

