use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects::definition::spawn_many;
use crate::objects::{cash_register, hero};

/// Construct the default environment to spawn in.
pub fn build_lobby(
    mut cmds: Commands,
    hero_assets: Res<hero::definition::HeroAssets>,
    cash_register_assets: Res<cash_register::definition::CashRegisterAssets>,
    asset_server: Res<AssetServer>,
) {
    spawn_many::<hero::definition::HeroAssets>(&mut cmds, &hero_assets, hero_spawns());
    spawn_many::<cash_register::definition::CashRegisterAssets>(
        &mut cmds,
        &cash_register_assets,
        cash_register_spawns(),
    );

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/worlde.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        lobby_map_transform(),
        Visibility::default(),
        RigidBody::Static,
    ));
}

fn hero_spawns() -> [Transform; 1] {
    [Transform::from_xyz(0.0, 67.0, 0.0)]
}

fn cash_register_spawns() -> [Transform; 1] {
    [Transform::from_rotation(Quat::from_euler(
        EulerRot::XYZ,
        0.0,
        (90_f32).to_radians(),
        0.0,
    ))
    .with_translation(Vec3::new(4.5, 1.25, 23.0))]
}

fn lobby_map_transform() -> Transform {
    Transform::from_xyz(0.0, 0.0, 0.0)
}
