use crate::objects::cash_register;
use crate::objects::hero;
use bevy::prelude::*;

pub fn paper_interaction(world: &mut World, entity: Entity) {
    info!("Paper Collected");
    world.trigger(hero::definition::AddScore(10));
    world.trigger(cash_register::definition::Kaching { register: entity });
    world.despawn(entity);
}
