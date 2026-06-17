use avian3d::prelude::*;
use bevy::prelude::*;

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
pub fn setup_oube(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    cmds.insert_resource(Oube {
        mesh: mesh.add(Cuboid::new(2.0, 2.0, 2.0)),
        mats: mats.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("images/boxtexture.jpeg")),
            ..default()
        }),
    });
}

/// Spawns a rigidbody cube at these coordinates.
pub fn spawn_oube(cmds: &mut Commands, oube: &Oube, pos: Vec3) {
    cmds.spawn((
        Mesh3d(oube.mesh.clone()),
        MeshMaterial3d(oube.mats.clone()),
        Transform::from_xyz(pos.x, pos.y + 2.0, pos.z),
        Collider::cuboid(2.0, 2.0, 2.0),
        RigidBody::Dynamic,
    ));
}

/// Spawns a cube through SpawnCubeEvent.
pub fn spawn_physics_cube(event: On<SpawnCubeEvent>, mut cmds: Commands, oube: Res<Oube>) {
    spawn_oube(&mut cmds, &oube, event.position);
}
