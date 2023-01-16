#![feature(test)]

mod quad_tree;
mod entity;
extern crate test;

use rand::*;
use entity::*;
use quad_tree::*;

const MODEL_SIZE: usize = 1000;
const WORLD_SIZE: u16 = 128;

pub fn init_model() -> Vec<Entity> {
    let mut model = Vec::new();
    let mut rng: rngs::StdRng = SeedableRng::seed_from_u64(42);
    for _ in 0..MODEL_SIZE {
        model.push(Entity {
            x: rng.gen_range(0..WORLD_SIZE),
            y: rng.gen_range(0..WORLD_SIZE),
            width: 1,
            height: 1,
            collision: false,
        });
    };
    println!("model {:?}", model);
    model
}

fn main() {
    let mut model = init_model();
    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    //the main game loop
    loop {

        rebuild_tree(&mut tree, &mut model);

        //update the entities
        for entity in model.iter_mut() {
            let local_model = tree.broad_phase_mut(entity.x, entity.y);
            update_entity_local(entity, local_model);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    //This is called once per frame by the bencher. It is the same as the main game loop
    fn heapless_tree_main(model: &mut Vec<Entity>, tree: &mut QuadTree<&mut Entity>) {
        
        rebuild_tree(tree, model);

        //update the entities
        for entity in model.iter_mut() {
            let local_model = tree.broad_phase_mut(entity.x, entity.y);
            update_entity_local(entity, local_model);
        }

    }

    //This is called once per frame by the bencher. It is the same as the main game loop
    fn no_broad_phase_main(model: &mut Vec<Entity>) {

        //update the entities
        for i in 0..model.len() {
            let entity = unsafe {&mut *(&mut model[i] as *mut Entity)};
            update_entity_global(entity, model);
        }

    }

    #[bench]
    fn bench_heapless_tree(b: &mut Bencher) {
        let mut model = init_model();
        let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

        b.iter(|| heapless_tree_main(&mut model, &mut tree))
    }

    #[bench]
    fn bench_no_broad_phase(b: &mut Bencher) {
        let mut model = init_model();
        b.iter(|| no_broad_phase_main(&mut model))
    }
}