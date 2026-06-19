use avian3d::prelude::*;
use bevy::prelude::*;

use super::logic;
use crate::build::layout::LobbyLayout;
use crate::objects;
use crate::objects::definition::Interactable;

#[derive(Resource)]
pub struct Paper {
    pub mesh: Handle<Mesh>,
    pub mats: Handle<StandardMaterial>,
    pub size: Vec3,
}

impl FromWorld for Paper {
    fn from_world(world: &mut World) -> Self {
        let size = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 0.25,
        };
        let mesh = {
            let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
            mesh_assets.add(Cuboid::new(size.x, size.y, size.z))
        };
        let mats = {
            let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
            material_assets.add(Color::srgb_u8(255, 255, 255))
        };

        Self {
            mesh,
            mats,
            size: Vec3::new(size.x, size.y, size.z),
        }
    }
}

impl objects::definition::ObjectBlueprint for Paper {
    type SpawnConfig = Vec3;

    fn spawn(cmds: &mut Commands, paper: &Self, pos: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            Mesh3d(paper.mesh.clone()),
            MeshMaterial3d(paper.mats.clone()),
            Transform::from_xyz(pos.x, pos.y + 2.5, pos.z),
            Collider::cuboid(paper.size.x, paper.size.y * 2.0, paper.size.z),
            Interactable {
                on_click: logic::paper_interaction,
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        ))
        .id()
    }
}

pub fn initialize_papers(mut cmds: Commands, paper: Res<Paper>, lobby_layout: Res<LobbyLayout>) {
    objects::definition::spawn_many(
        &mut cmds,
        paper.as_ref(),
        lobby_layout.sample_paper_spawns(),
    );
}
