use crate::objects::hero;
use bevy::prelude::*;

pub fn paper_interaction(world: &mut World, entity: Entity) {
    info!("Paper Collected");
    world.trigger(hero::definition::AddScore(10));
    world.despawn(entity);
}
