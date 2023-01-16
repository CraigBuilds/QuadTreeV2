mod quad_tree;
use quad_tree::*;
mod entity;
use entity::*;

fn main() {
    
    let mut model = vec![
        Entity{x: 0, y: 0, width: 2, height: 2, collision: false},
        Entity{x: 1, y: 1, width: 2, height: 2, collision: false},
        Entity{x: 20, y: 20, width: 2, height: 2, collision: false},
        Entity{x: 21, y: 21, width: 2, height: 2, collision: false},
    ];

    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    '_main: loop {

        rebuild_tree(&mut tree, &mut model);

        //update the entities
        for entity in model.iter_mut() {
            let local_model = &mut tree.get_mut_leaf_around(entity.x, entity.y).unwrap().data;
            update_entity(entity, local_model);
        }
    }
}