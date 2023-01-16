use super::{GetX, GetY};
use rand::*;

#[derive(Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub collision: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ModelConfig {
    pub model_size: u16,
    pub world_size: u16,
}
impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_size: 1000,
            world_size: 128,
        }
    }
}

pub fn init_model(config: ModelConfig) -> Vec<Entity> {
    let mut model = Vec::new();
    let mut rng: rngs::StdRng = SeedableRng::seed_from_u64(42);
    for _ in 0..config.model_size {
        model.push(Entity {
            x: rng.gen_range(0..config.world_size),
            y: rng.gen_range(0..config.world_size),
            width: 1,
            height: 1,
            collision: false,
        });
    };
    model
}

impl GetX for Entity {
    fn get_x(&self) -> u16 {
        self.x
    }
}

impl GetY for Entity {
    fn get_y(&self) -> u16 {
        self.y
    }
}

//TODO: More things than just collision checks. How does the QuadTree performance scale as the complexity of the update function increases?

pub fn update_entity_local(entity: &mut Entity, local_model: &mut [&mut Entity]) {
    for other_entity in local_model {
        if is_coliding(entity, other_entity) {
            entity.collision = true;
        }
    }
}

#[allow(dead_code)]
pub fn update_entity_global(entity: &mut Entity, model: &mut [Entity]) {
    for other_entity in model {
        //skip self checks
        if entity as *const Entity == other_entity as *const Entity {
            continue;
        }
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

