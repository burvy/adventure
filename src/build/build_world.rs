use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects::{cash_register, hero};

/// Construct the default environment to spawn in.
pub fn build_lobby(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // "hero"
    hero::definition::spawn_hero(&mut cmds, &mut mesh, &mut mats);

    cash_register::definition::spawn_cash_register(&mut cmds, &asset_server);

    // fps map
    cmds.spawn((
        SceneRoot(asset_server.load("models/map/worlde.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        RigidBody::Static,
    ));
}
