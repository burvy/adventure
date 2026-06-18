use avian3d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

use crate::objects::definition::{ObjectBlueprint, spawn_many};

const LOBBY_LAYOUT_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/layouts/lobby.ron"
));

#[derive(Clone)]
pub struct StaticScenePlacement {
    pub scene_path: String,
    pub transform: Transform,
}

#[derive(Resource)]
pub struct LobbyLayout {
    hero_spawns: Vec<Transform>,
    cash_register_spawns: Vec<Transform>,
    static_scenes: Vec<StaticScenePlacement>,
}

impl FromWorld for LobbyLayout {
    fn from_world(_world: &mut World) -> Self {
        let parsed: LobbyLayoutFile = ron::from_str(LOBBY_LAYOUT_SOURCE)
            .expect("lobby.ron must contain a valid lobby layout");

        parsed.into()
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

#[derive(Deserialize)]
struct LobbyLayoutFile {
    hero_spawns: Vec<TransformDef>,
    cash_register_spawns: Vec<TransformDef>,
    static_scenes: Vec<StaticScenePlacementDef>,
}

impl From<LobbyLayoutFile> for LobbyLayout {
    fn from(layout: LobbyLayoutFile) -> Self {
        Self {
            hero_spawns: layout
                .hero_spawns
                .into_iter()
                .map(TransformDef::into_transform)
                .collect(),
            cash_register_spawns: layout
                .cash_register_spawns
                .into_iter()
                .map(TransformDef::into_transform)
                .collect(),
            static_scenes: layout
                .static_scenes
                .into_iter()
                .map(StaticScenePlacementDef::into_runtime)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
struct StaticScenePlacementDef {
    scene_path: String,
    transform: TransformDef,
}

impl StaticScenePlacementDef {
    fn into_runtime(self) -> StaticScenePlacement {
        StaticScenePlacement {
            scene_path: self.scene_path,
            transform: self.transform.into_transform(),
        }
    }
}

#[derive(Deserialize)]
struct TransformDef {
    translation: Vec3Def,
    #[serde(default = "zero_rotation")]
    rotation_degrees: Vec3Def,
    #[serde(default = "unit_scale")]
    scale: Vec3Def,
}

impl TransformDef {
    fn into_transform(self) -> Transform {
        Transform {
            translation: self.translation.into_vec3(),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                self.rotation_degrees.x.to_radians(),
                self.rotation_degrees.y.to_radians(),
                self.rotation_degrees.z.to_radians(),
            ),
            scale: self.scale.into_vec3(),
        }
    }
}

#[derive(Deserialize)]
struct Vec3Def {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3Def {
    fn into_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

fn zero_rotation() -> Vec3Def {
    Vec3Def {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn unit_scale() -> Vec3Def {
    Vec3Def {
        x: 1.0,
        y: 1.0,
        z: 1.0,
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
            SceneRoot(asset_server.load(&placement.scene_path)),
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            placement.transform,
            Visibility::default(),
            RigidBody::Static,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lobby_layout_source_parses() {
        match ron::from_str::<LobbyLayoutFile>(LOBBY_LAYOUT_SOURCE) {
            Ok(_) => {}
            Err(error) => panic!("failed to parse lobby.ron: {error}"),
        }
    }
}
