use bevy::prelude::*;

pub fn paper_interaction(world: &mut World, entity: Entity) {
    info!("Paper Collected");
    world.despawn(entity);
}
