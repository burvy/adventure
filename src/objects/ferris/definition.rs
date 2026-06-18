use avian3d::math::*;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty::definition::WantMove;
use crate::objects::{self, ferris::logic};

#[derive(Event)]
pub struct SpawnFerrisesEvent;

#[derive(Resource)]
pub struct Ferris {
    pub scene: Handle<Scene>,
    pub bounds: Vec3,
}

impl FromWorld for Ferris {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            scene: asset_server.load("models/crab.glb#Scene0"),
            bounds: Vec3::new(3.0, 2.0, 3.0),
        }
    }
}

impl objects::definition::ObjectBlueprint for Ferris {
    type SpawnConfig = Transform;

    fn spawn(cmds: &mut Commands, ferris: &Self, transform: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            SceneRoot(ferris.scene.clone()),
            Collider::cuboid(ferris.bounds.x, ferris.bounds.y, ferris.bounds.z),
            transform,
            Visibility::default(),
            RigidBody::Dynamic,
            objects::definition::Interactable {
                on_click: logic::on_click,
            },
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
        ))
        .id()
    }
}

pub fn spawn_ferrises(_event: On<SpawnFerrisesEvent>, mut cmds: Commands, ferris: Res<Ferris>) {
    objects::definition::spawn_many(
        &mut cmds,
        ferris.as_ref(),
        (0..10).map(|_| Transform::from_xyz(0.0, 20.0, 0.0).with_scale(Vec3::splat(0.15))),
    );
}
