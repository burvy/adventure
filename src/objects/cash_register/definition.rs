use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects::{self, cash_register::logic};

#[derive(Resource)]
pub struct CashRegisterAssets {
    scene: Handle<Scene>,
}

impl FromWorld for CashRegisterAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            scene: asset_server.load("models/register.glb#Scene0"),
        }
    }
}

impl objects::definition::ObjectBlueprint for CashRegisterAssets {
    type SpawnConfig = Transform;

    fn spawn(cmds: &mut Commands, blueprint: &Self, transform: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            SceneRoot(blueprint.scene.clone()),
            Collider::cuboid(1.0, 1.0, 1.0),
            transform,
            Visibility::default(),
            objects::definition::Interactable {
                on_click: logic::on_click,
            },
            RigidBody::Static,
            children![(
                PointLight {
                    intensity: 240000.0,
                    ..Default::default()
                },
                objects::definition::Visible(false),
                Transform::from_xyz(0.0, 5.0, 0.0),
            )],
        ))
        .id()
    }
}
