use criterion::{criterion_group, criterion_main, Criterion};
use rust_quadmap_v2::entity::*;
use rust_quadmap_v2::quad_tree::*;
use rust_quadmap_v2::enum_quadtree::QuadTree as EnumQuadTree;
use rust_quadmap_v2::enum_quadtree::rebuild_tree as rebuild_tree_enum;

//This is called once per frame by the bencher. It is the same as the main game loop
fn no_broad_phase_main(model: &mut Vec<Entity>) {

    //update the entities
    for i in 0..model.len() {
        let entity = unsafe {&mut *(&mut model[i] as *mut Entity)};
        update_entity_global(entity, model);
    }

}

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
fn enum_tree_main(model: &mut Vec<Entity>, tree: &mut EnumQuadTree<&mut Entity>) {
    
    rebuild_tree_enum(tree, model);

    //update the entities
    for entity in model.iter_mut() {
        let local_model = tree.broad_phase_mut(entity.x, entity.y);
        update_entity_local(entity, local_model);
    }

}

//TODO https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html#benchmarking-with-a-range-of-values
//TODO test with differnt depths, entity size variance, and world sizes

fn bench_no_broad_phase(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    c.bench_function(&format!("no_broad_phase_main({:?})", config), |b| b.iter(|| no_broad_phase_main(&mut model)));
}

fn bench_heapless_tree(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16
    c.bench_function(&format!("heapless_tree_main({:?})", config), |b| b.iter(|| heapless_tree_main(&mut model, &mut tree)));
}

fn bench_enum_tree(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    let mut tree = EnumQuadTree::new(0,0,128,128, 3); //128x128 world, 8x8 grid, so every leaf is 16x16
    c.bench_function(&format!("enum_tree_main({:?})", config), |b| b.iter(|| enum_tree_main(&mut model, &mut tree)));
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_no_broad_phase, bench_heapless_tree, bench_enum_tree
);

criterion_main!(benches);