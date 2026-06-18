use avian3d::math::*;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty::definition::WantMove;
use crate::objects;

#[derive(Event)]
pub struct SpawnFerrisesEvent;

#[derive(Resource)]
pub struct Ferris {
    pub scene: Handle<Scene>,
    pub bounds: Vec3,
}

pub fn setup_ferris(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(Ferris {
        scene: asset_server.load("models/crab.glb#Scene0"),
        bounds: Vec3::new(3.0, 2.0, 3.0),
    });
}

pub fn spawn_ferris(cmds: &mut Commands, ferris: &Ferris) {
    cmds.spawn((
        SceneRoot(ferris.scene.clone()),
        Collider::cuboid(ferris.bounds.x, ferris.bounds.y, ferris.bounds.z),
        Transform::from_xyz(0.0, 20.0, 0.0).with_scale(Vec3::new(0.15, 0.15, 0.15)),
        Visibility::default(),
        RigidBody::Dynamic,
        objects::definition::Thing::Ferris,
        WantMove {
            zinput: 0,
            xinput: 0,
            jump: false,
            forward: Vec3::Z,
            move_speed: 5.0,
        },
        ShapeCaster::new(
            Collider::cuboid(ferris.bounds.x, ferris.bounds.y, ferris.bounds.z),
            Vector::ZERO,
            Quaternion::default(),
            Dir3::NEG_Y,
        )
        .with_max_distance(1.0),
    ));
}

pub fn spawn_ferrises(_event: On<SpawnFerrisesEvent>, mut cmds: Commands, ferris: Res<Ferris>) {
    for _ in 0..10 {
        spawn_ferris(&mut cmds, &ferris);
    }
}
