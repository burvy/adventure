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
        Collider::sphere(3.0),
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
        ShapeCaster::new(
            Collider::capsule(1.5, 3.0),
            Vector::ZERO,
            Quaternion::default(),
            Dir3::NEG_Y,
        )
        .with_max_distance(0.15),
        // we want to target the forward cast on each of the ferris entities
        children![
            ShapeCaster::new(
                Collider::cuboid(1.0, 1.0, 1.0),
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Z,
            )
            .with_max_distance(5.0),
            objects::definition::ForwardCast
        ],
    ));
}

pub fn spawn_ferrises(_event: On<SpawnFerrisesEvent>, mut cmds: Commands, ferris: &Res<Ferris>) {
    for _ in 0..25 {
        spawn_ferris(&mut cmds, ferris);
    }
}
