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

#[derive(Clone)]
pub struct ScatterRegion {
    pub count: usize,
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Clone)]
pub struct RepeatedTransformPlacement {
    pub count: usize,
    pub transform: Transform,
}

#[derive(Resource)]
pub struct LobbyLayout {
    hero_spawns: Vec<Transform>,
    cash_register_spawns: Vec<Transform>,
    paper_scatter_regions: Vec<ScatterRegion>,
    ferris_spawn_groups: Vec<RepeatedTransformPlacement>,
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

    pub fn sample_paper_spawns(&self) -> Vec<Vec3> {
        let mut spawns = Vec::new();

        for region in &self.paper_scatter_regions {
            for _ in 0..region.count {
                spawns.push(Vec3::new(
                    sample_component(region.min.x, region.max.x),
                    sample_component(region.min.y, region.max.y),
                    sample_component(region.min.z, region.max.z),
                ));
            }
        }

        spawns
    }

    pub fn ferris_spawns(&self) -> Vec<Transform> {
        let mut spawns = Vec::new();

        for group in &self.ferris_spawn_groups {
            for _ in 0..group.count {
                spawns.push(group.transform.clone());
            }
        }

        spawns
    }

    pub fn static_scenes(&self) -> &[StaticScenePlacement] {
        &self.static_scenes
    }
}

#[derive(Deserialize)]
struct LobbyLayoutFile {
    hero_spawns: Vec<TransformDef>,
    cash_register_spawns: Vec<TransformDef>,
    paper_scatter_regions: Vec<ScatterRegionDef>,
    ferris_spawn_groups: Vec<RepeatedTransformPlacementDef>,
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
            paper_scatter_regions: layout
                .paper_scatter_regions
                .into_iter()
                .map(ScatterRegionDef::into_runtime)
                .collect(),
            ferris_spawn_groups: layout
                .ferris_spawn_groups
                .into_iter()
                .map(RepeatedTransformPlacementDef::into_runtime)
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
struct ScatterRegionDef {
    count: usize,
    min: Vec3Def,
    max: Vec3Def,
}

impl ScatterRegionDef {
    fn into_runtime(self) -> ScatterRegion {
        ScatterRegion {
            count: self.count,
            min: self.min.into_vec3(),
            max: self.max.into_vec3(),
        }
    }
}

#[derive(Deserialize)]
struct RepeatedTransformPlacementDef {
    count: usize,
    transform: TransformDef,
}

impl RepeatedTransformPlacementDef {
    fn into_runtime(self) -> RepeatedTransformPlacement {
        RepeatedTransformPlacement {
            count: self.count,
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

fn sample_component(min: f32, max: f32) -> f32 {
    if (max - min).abs() <= f32::EPSILON {
        min
    } else {
        rand::random_range(min..max)
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
