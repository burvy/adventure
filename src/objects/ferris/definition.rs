use avian3d::math::*;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty::definition::WantMove;
use crate::build::layout::LobbyLayout;
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
        // used .clone to bypass the borrow checker here to add mesh and mats, change later if desired
        let asset_server = world.resource::<AssetServer>().clone();

        let bounds = Vec3::new(10.0, 10.0, 10.0);

        Self {
            scene: asset_server.load("models/crab.glb#Scene0"),
            bounds: bounds,
        }
    }
}

impl objects::definition::ObjectBlueprint for Ferris {
    type SpawnConfig = Transform;

    fn spawn(cmds: &mut Commands, ferris: &Self, transform: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            // The root entity holds the collider, physics, and main position
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
        .with_children(|parent| {
            parent.spawn((
                SceneRoot(ferris.scene.clone()),
                Transform::from_xyz(0.0, -6.7, 0.0),
            ));
        })
        .id()
    }
}

pub fn spawn_ferrises(
    _event: On<SpawnFerrisesEvent>,
    mut cmds: Commands,
    ferris: Res<Ferris>,
    lobby_layout: Res<LobbyLayout>,
) {
    objects::definition::spawn_many(&mut cmds, ferris.as_ref(), lobby_layout.ferris_spawns());
}
