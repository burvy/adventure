use crate::objects;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::build::build_cube;
use crate::objects::hero::definition;

pub fn add_score(trigger: On<definition::AddScore>, mut hero: Single<&mut definition::Hero>) {
    hero.score += trigger.event().0;
    info!("Hero Score is now {}", hero.score);
}

/// Stuff to run when left clicks are detected.
pub fn on_click(
    ray: &RayCaster,
    hits: &RayHits,
    cmds: &mut Commands,
    interacts: &Query<&objects::definition::Interactable>,
) {
    // more idiomatic way to check for hits
    if let Some(hit) = hits.iter_sorted().next() {
        if let Ok(interactable) = interacts.get(hit.entity) {
            let on_click = interactable.on_click;
            cmds.queue(move |world: &mut World| {
                (on_click)(world, hit.entity);
            });
        } else {
            cmds.trigger(build_cube::SpawnCubeEvent::new(spawn_loc(
                get_impact(ray.global_origin(), ray.global_direction(), hit.distance),
                hit.normal,
                1.0,
            )))
        }
    }
}
fn get_impact(origin: Vec3, direction: Dir3, distance: f32) -> Vec3 {
    origin + direction * distance
}
fn spawn_loc(impact_pos: Vec3, normal: Vec3, added: f32) -> Vec3 {
    impact_pos + normal * added
}
