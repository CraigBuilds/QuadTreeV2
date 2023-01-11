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

        for local_model in into_iter(&tree) {
            for i in 0..local_model.len() {
                let (entity, leading, trailing) = split_at_rest(local_model, i);
                let others = chain_others(leading, trailing);
                update_entity(entity, others);
            }
        }
        println!("");
    }
}

fn update_entity<'a>(_entity: &&mut Entity, _local_model: impl Iterator<Item = &'a &'a mut Entity>) {

}

fn is_coliding(entity: &Entity, other_entity: &Entity) -> bool {
    entity.x < other_entity.x + other_entity.width &&
    entity.x + entity.width > other_entity.x &&
    entity.y < other_entity.y + other_entity.height &&
    entity.y + entity.height > other_entity.y
}

pub fn split_at_rest_mut<T>(x: &mut [T], index: usize) -> (&mut T, &mut [T], &mut [T]) {
    debug_assert!(index < x.len());
    let (leading, trailing) = x.split_at_mut(index); //TODO unchecked version?
    let (val, trailing) = trailing.split_first_mut().unwrap();
    (val, leading, trailing)
}

pub fn split_at_rest<T>(x: & [T], index: usize) -> (& T, & [T], & [T]) {
    debug_assert!(index < x.len());
    let (leading, trailing) = x.split_at(index); //TODO unchecked version?
    let (val, trailing) = trailing.split_first().unwrap();
    (val, leading, trailing)
}


pub fn chain_others_mut<'a, T>(leading: &'a mut [T], trailing: &'a mut [T]) -> impl Iterator<Item = &'a mut T> {
    leading.iter_mut().chain(trailing.iter_mut())
}

pub fn chain_others<'a, T>(leading: &'a [T], trailing: &'a [T]) -> impl Iterator<Item = &'a T> {
    leading.iter().chain(trailing.iter())
}