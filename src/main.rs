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

    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    loop {

        //rebuild the tree (this does not effect underlying vec capacities)
        tree.clear();
        for entity in model.iter() {
            //insert a reference to the entity into the tree
            tree.insert(entity.x, entity.y, entity);
        }

        //calculate colisions
        for entity in model.iter_mut() {
            let leaf = tree.get_leaf_around(entity.x, entity.y).unwrap();
            for (_,_,other_entity) in leaf.vec.iter() {
                if is_coliding(entity, other_entity) {
                    entity.collision = true;
                }
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

