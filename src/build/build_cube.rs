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
) {
    cmds.insert_resource(Oube {
        mesh: mesh.add(Cuboid::new(1.0, 1.0, 1.0)),
        mats: mats.add(Color::srgb_u8(255, 102, 0)),
    });
}

/// Spawns a rigidbody cube at these coordinates.
pub fn spawn_oube(cmds: &mut Commands, oube: &Oube, x: f32, y: f32, z: f32) {
    cmds.spawn((
        Mesh3d(oube.mesh.clone()),
        MeshMaterial3d(oube.mats.clone()),
        Transform::from_xyz(x, y, z),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
}

/// Spawns a cube through SpawnCubeEvent.
pub fn spawn_physics_cube(event: On<SpawnCubeEvent>, mut cmds: Commands, oube: Res<Oube>) {
    spawn_oube(
        &mut cmds,
        &oube,
        event.position.x,
        event.position.y,
        event.position.z,
    );
}
