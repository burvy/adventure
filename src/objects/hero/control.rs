use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW, Space};
use bevy::prelude::*;

use avian3d::prelude::*;

use crate::almighty;
use crate::almighty::definition::WantMove;
use crate::objects::hero::definition::Hero;
use crate::objects::hero::definition::HeroBody;
use crate::objects::hero::definition::HeroCamera;
use crate::objects::hero::logic;

use crate::objects;

/// Reads the hero's input and sets where they want to move.
pub fn hero_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hero: Single<(&mut Hero, &mut WantMove, &ShapeHits), With<Hero>>,
) {
    // tuple destructuring, this does not create side-effects
    let (hero, want_move, collisions) = &mut *hero;

    want_move.zinput = (keys.pressed(KeyW) as i8) - (keys.pressed(KeyS) as i8);
    want_move.xinput = (keys.pressed(KeyD) as i8) - (keys.pressed(KeyA) as i8);

    if keys.pressed(Space) {
        want_move.jump = almighty::logic::validate_jump(collisions);
    }

    // store the forward direction to be used later on
    // -Z is forward in bevy
    want_move.forward = Vec3::new(-hero.yaw.sin(), 0.0, -hero.yaw.cos());
}

/// Detects when this instance left clicks.
pub fn hero_left_click(
    mut cmds: Commands,
    click: Res<ButtonInput<MouseButton>>,
    cast: Single<(&RayCaster, &RayHits), With<crate::objects::hero::definition::DebugTool>>,
    interacts: Query<&objects::definition::Interactable>,
) {
    if click.just_pressed(MouseButton::Left) {
        let (ray, hits) = &*cast;
        logic::on_click(ray, hits, &mut cmds, &interacts);
    }
}

/// Updates the player's stored rotation from mouse movement.
pub fn read_camera(mot: Res<AccumulatedMouseMotion>, mut hero: Single<&mut Hero>) {
    if hero.paused {
        return;
    }
    hero.pitch = (hero.pitch - mot.delta.y * hero.sens.y)
        .clamp(-(89.9_f32).to_radians(), (89.9_f32).to_radians());
    hero.yaw = (hero.yaw - mot.delta.x * hero.sens.x).rem_euclid(std::f32::consts::TAU);
}

/// Applies stored rotation to the body yaw of the hero.
pub fn update_body(mut hbod: Single<&mut Transform, With<HeroBody>>, hero: Single<&Hero>) {
    hbod.rotation = Quat::from_rotation_y(hero.yaw);
}

/// Applies stored rotation to the camera of the hero
pub fn update_camera(mut hcam: Single<&mut Transform, With<HeroCamera>>, hero: Single<&Hero>) {
    // actually apply transformation.
    hcam.rotation = Quat::from_euler(EulerRot::YXZ, hero.yaw, hero.pitch, 0.0);
}
