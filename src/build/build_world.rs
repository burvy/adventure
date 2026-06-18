use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects::definition::spawn_object;
use crate::objects::{cash_register, hero};

/// Construct the default environment to spawn in.
pub fn build_lobby(
    mut cmds: Commands,
    hero_assets: Res<hero::definition::HeroAssets>,
    cash_register_assets: Res<cash_register::definition::CashRegisterAssets>,
    asset_server: Res<AssetServer>,
) {
    spawn_object::<hero::definition::HeroAssets>(
        &mut cmds,
        &hero_assets,
        Transform::from_xyz(0.0, 67.0, 0.0),
    );
    spawn_object::<cash_register::definition::CashRegisterAssets>(
        &mut cmds,
        &cash_register_assets,
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            (90_f32).to_radians(),
            0.0,
        ))
        .with_translation(Vec3::new(4.5, 1.25, 23.0)),
    );

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/worlde.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        RigidBody::Static,
    ));
}
