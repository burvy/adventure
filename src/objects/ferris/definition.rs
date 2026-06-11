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
}

pub fn setup_ferris(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(Ferris {
        scene: asset_server.load("models/crab.glb#Scene0"),
    });
}

pub fn spawn_ferris(cmds: &mut Commands, ferris: &Ferris) {
    cmds.spawn((
        SceneRoot(ferris.scene.clone()),
        Collider::cuboid(3.0, 3.0, 3.0),
        Transform::from_xyz(0.0, 20.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        Visibility::default(),
        RigidBody::Dynamic,
        objects::definition::Thing::Ferris,
        WantMove {
            zinput: 0,
            xinput: 0,
            jump: false,
            forward: Vec3::Z,
            move_speed: 3.0,
        },
    ));
}

pub fn spawn_ferrises(_event: On<SpawnFerrisesEvent>, mut cmds: Commands, ferris: Res<Ferris>) {
    for _ in 0..25 {
        spawn_ferris(&mut cmds, &ferris);
    }
}
