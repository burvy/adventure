use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects;

/// Event to spawn a cube easily
/// Just do
#[derive(Event)]
pub struct SpawnCubeEvent {
    position: Vec3,
}

impl SpawnCubeEvent {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

#[derive(Resource)]
pub struct Oube {
    pub mesh: Handle<Mesh>,
    pub mats: Handle<StandardMaterial>,
}

impl FromWorld for Oube {
    fn from_world(world: &mut World) -> Self {
        let texture = {
            let asset_server = world.resource::<AssetServer>();
            asset_server.load("images/boxtexture.jpeg")
        };
        let mesh = {
            let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
            mesh_assets.add(Cuboid::new(2.0, 2.0, 2.0))
        };
        let mats = {
            let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
            material_assets.add(StandardMaterial {
                base_color_texture: Some(texture),
                ..default()
            })
        };

        Self { mesh, mats }
    }
}

impl objects::definition::ObjectBlueprint for Oube {
    type SpawnConfig = Vec3;

    fn spawn(cmds: &mut Commands, oube: &Self, pos: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            Mesh3d(oube.mesh.clone()),
            MeshMaterial3d(oube.mats.clone()),
            Transform::from_xyz(pos.x, pos.y + 2.0, pos.z),
            Collider::cuboid(2.0, 2.0, 2.0),
            RigidBody::Dynamic,
        ))
        .id()
    }
}

/// Spawns a cube through SpawnCubeEvent.
pub fn spawn_physics_cube(event: On<SpawnCubeEvent>, mut cmds: Commands, oube: Res<Oube>) {
    objects::definition::spawn_object::<Oube>(&mut cmds, &oube, event.position);
}
