use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects::definition::{ObjectBlueprint, spawn_many};

#[derive(Clone)]
pub struct StaticScenePlacement {
    pub scene_path: &'static str,
    pub transform: Transform,
}

#[derive(Resource)]
pub struct LobbyLayout {
    hero_spawns: Vec<Transform>,
    cash_register_spawns: Vec<Transform>,
    static_scenes: Vec<StaticScenePlacement>,
}

impl Default for LobbyLayout {
    fn default() -> Self {
        Self {
            hero_spawns: vec![Transform::from_xyz(0.0, 67.0, 0.0)],
            cash_register_spawns: vec![
                Transform::from_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    (90_f32).to_radians(),
                    0.0,
                ))
                .with_translation(Vec3::new(4.5, 1.25, 23.0)),
            ],
            static_scenes: vec![StaticScenePlacement {
                scene_path: "models/map/worlde.glb#Scene0",
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
            }],
        }
    }
}

impl LobbyLayout {
    pub fn hero_spawns(&self) -> &[Transform] {
        &self.hero_spawns
    }

    pub fn cash_register_spawns(&self) -> &[Transform] {
        &self.cash_register_spawns
    }

    pub fn static_scenes(&self) -> &[StaticScenePlacement] {
        &self.static_scenes
    }
}

pub fn spawn_layout<T>(cmds: &mut Commands, blueprint: &T, placements: &[T::SpawnConfig])
where
    T: ObjectBlueprint,
    T::SpawnConfig: Clone,
{
    spawn_many::<T>(cmds, blueprint, placements.iter().cloned());
}

pub fn spawn_static_scenes(
    cmds: &mut Commands,
    asset_server: &AssetServer,
    placements: &[StaticScenePlacement],
) {
    for placement in placements {
        cmds.spawn((
            SceneRoot(asset_server.load(placement.scene_path)),
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            placement.transform,
            Visibility::default(),
            RigidBody::Static,
        ));
    }
}
