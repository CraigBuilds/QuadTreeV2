#![feature(test)]

pub mod fixed_depth_quad_tree;
pub mod enum_quadtree;
pub mod entity;
use entity::*;

pub trait GetX {fn get_x(&self) -> u16;}
pub trait GetY {fn get_y(&self) -> u16;}

pub fn example_main() {
    
    use fixed_depth_quad_tree::*;

    let mut model = init_model(Default::default());
    let mut tree = QuadTree::new_empty(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    //the main game loop
    loop {

        rebuild_from_model(&mut tree, &mut model);

        //update the entities
        for entity in model.iter_mut() {
            let local_model = tree.broad_phase_mut(entity.x, entity.y);
            update_entity_local(entity, local_model);
        }
    }
}

pub fn example_main_2() {
    
    use enum_quadtree::*;

    let mut model = init_model(Default::default());
    let mut tree = QuadTree::new_empty(0,0,128,128, 3); //128x128 world, 8x8 grid, so every leaf is 16x16

    //the main game loop
    loop {

        rebuild_from_model(&mut tree, &mut model);

        //update the entities
        for entity in model.iter_mut() {
            let local_model = tree.broad_phase_mut(entity.x, entity.y);
            update_entity_local(entity, local_model);
        }
    }
}