use criterion::{criterion_group, criterion_main, Criterion};
use rust_quadmap_v2::entity::*;
use rust_quadmap_v2::fixed_depth_quad_tree::*;
use rust_quadmap_v2::enum_quadtree::{
    QuadTree as EnumQuadTree,
    rebuild_from_model as rebuild_from_model_enum,
    build_from_model as build_from_model_enum,
};

//// The _main functions are called once per frame by the bencher. This is the same behavior as
//// if they are in an outer loop (main game loop).


fn no_broad_phase_main(model: &mut Vec<Entity>) {

    //update the entities
    for i in 0..model.len() {
        ////SAFETY This is safe because update_entity_global checks if the entity is the same as the other_entity
        let entity = unsafe {&mut *(&mut model[i] as *mut Entity)};
        update_entity_global(entity, model);
    }

}

fn fixed_depth_tree_main(model: &mut Vec<Entity>, tree: &mut QuadTree<&mut Entity>) {
    
    rebuild_from_model(tree, model);

    //update the entities
    for entity in model.iter_mut() {
        let local_model = tree.broad_phase_mut(entity.x, entity.y);
        update_entity_local(entity, local_model);
    }

}

fn fixed_depth_tree_no_cache_main(model: &mut Vec<Entity>) {
    
    let mut tree = QuadTree::new_empty(0, 0, 128, 128);
    ////SAFETY This is safe because the tree is empty
    unsafe { build_from_model(&mut tree, model) };

    //update the entities
    for entity in model.iter_mut() {
        let local_model = tree.broad_phase_mut(entity.x, entity.y);
        update_entity_local(entity, local_model);
    }

}

fn enum_tree_main(model: &mut Vec<Entity>, tree: &mut EnumQuadTree<&mut Entity>) {
    
    rebuild_from_model_enum(tree, model);

    //update the entities
    for entity in model.iter_mut() {
        let local_model = tree.broad_phase_mut(entity.x, entity.y);
        update_entity_local(entity, local_model);
    }

}

fn enum_tree_no_cache_main(model: &mut Vec<Entity>) {
    
    let mut tree = EnumQuadTree::new_empty(0, 0, 128, 128, 3);
    ////SAFETY This is safe because the tree is empty
    unsafe{ build_from_model_enum(&mut tree, model) };

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

fn bench_fixed_depth_tree(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    let mut tree = QuadTree::new_empty(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16
    c.bench_function(&format!("fixed_depth_tree_main({:?})", config), |b| b.iter(|| fixed_depth_tree_main(&mut model, &mut tree)));
}

fn bench_fixed_depth_tree_no_cache(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    c.bench_function(&format!("fixed_depth_tree_no_cache_main({:?})", config), |b| b.iter(|| fixed_depth_tree_no_cache_main(&mut model)));
}

fn bench_enum_tree(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    let mut tree = EnumQuadTree::new_empty(0,0,128,128, 3); //128x128 world, 8x8 grid, so every leaf is 16x16
    c.bench_function(&format!("enum_tree_main({:?})", config), |b| b.iter(|| enum_tree_main(&mut model, &mut tree)));
}

fn bench_enum_tree_no_cache(c: &mut Criterion) {
    let config = ModelConfig{model_size: 1000, world_size: 128};
    let mut model = init_model(config);
    c.bench_function(&format!("enum_tree_no_cache_main({:?})", config), |b| b.iter(|| enum_tree_no_cache_main(&mut model)));
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_no_broad_phase, bench_fixed_depth_tree, bench_enum_tree, bench_fixed_depth_tree_no_cache, bench_enum_tree_no_cache
);

criterion_main!(benches);