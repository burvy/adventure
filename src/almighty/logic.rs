use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty::definition::Boing;
use crate::almighty::definition::WantMove;
use crate::objects;

use trig_const::cos;

/// Jumping velocity impulse.
const JUMP_STRENGTH: f32 = 10.0;

/// The angle between the entity and the ground that jumping should be possible at
const JUMP_ANGLE_RAW: f64 = std::f64::consts::FRAC_PI_3;

/// Valid jump angle but cosined
const JUMP_ANGLE: f32 = cos(JUMP_ANGLE_RAW) as f32;

/// Fulfill the movement wants of all entities and move them by applying velocity.
pub fn move_all(mut cmds: Commands, query: Query<(Entity, &mut WantMove, &mut LinearVelocity)>) {
    for (entity, mut des, mut velocity) in query {
        *velocity = LinearVelocity((local_dir(&des) * get_speed(&des)).with_y(velocity.y));
        if des.jump {
            des.jump = false;
            velocity.y = JUMP_STRENGTH;
            // trigger the boing sound and hopefully we know where this entity is too
            cmds.trigger(Boing { jumper: entity });
        }
    }
}
/// Returns true if the collision list you passed in implies that you can jump.
pub fn validate_jump(collisions: &ShapeHits) -> bool {
    // iterate through the collisions list and find any valid hit, returns boolean
    collisions.iter().any(|hit| {
        // hit.normal2 is negative to flip the normal around towards the player
        // normal2 is the ground, check if the normal is JUMP_ANGLE degrees to player or lower,
        // then it is walkable
        -hit.normal2.y >= JUMP_ANGLE
    })
}
/// Set things visible or invisible based on the custom visibility tag
pub fn update_visibilities(
    visibilizables: Query<(&mut Visibility, &objects::definition::Visible)>,
) {
    for (mut visibility, visibilizable) in visibilizables {
        *visibility = if visibilizable.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Gives you a local horizontal movement vector.
fn local_dir(want_move: &WantMove) -> Vec3 {
    want_move.forward * (want_move.zinput as f32)
        + Vec3::new(-want_move.forward.z, 0.0, want_move.forward.x) * (want_move.xinput as f32)
}
/// Returns the movement speed specified by WantMove for horizontal movement.
fn get_speed(want_move: &WantMove) -> f32 {
    want_move.move_speed
}

pub fn on_jump_sound(
    trigger: On<Boing>,
    asset_server: Res<AssetServer>,
    mut cmds: Commands,
    transforms: Query<&Transform>,
) {
    let boing_handle = asset_server.load("sounds/boing.wav");

    let jump_event = trigger.event();
    let position = transforms
        .get(jump_event.jumper)
        .cloned()
        .unwrap_or_default();

    cmds.spawn((
        AudioPlayer::new(boing_handle),
        PlaybackSettings::DESPAWN.with_spatial(true),
        position,
    ));
}
