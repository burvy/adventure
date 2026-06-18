use avian3d::math::*;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty::definition::WantMove;
use crate::objects;

/// A tag to identify the singular hero. Every player is a hero in their own instance.
/// There should never be more than one hero. Everyone else is not a hero.
#[derive(Component)]
pub struct Hero {
    /// Stores if the hero is in a paused state.
    pub paused: bool,
    /// Mouse sensitivity of the hero.
    pub sens: Vec2,
    /// Pitch rotation value of the hero to be applied to HeroCamera.
    pub pitch: f32,
    /// Yaw rotation value of the hero to be applied to HeroCamera.
    pub yaw: f32,
}

/// Tag the camera
#[derive(Component)]
pub struct HeroCamera;

/// Tag the body
#[derive(Component)]
pub struct HeroBody;

#[derive(Component)]
pub struct DebugTool;

#[derive(Resource)]
pub struct HeroAssets {
    arm_mesh: Handle<Mesh>,
    arm_material: Handle<StandardMaterial>,
}

impl FromWorld for HeroAssets {
    fn from_world(world: &mut World) -> Self {
        let arm_mesh = {
            let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
            mesh_assets.add(Cuboid::new(0.5, 0.5, 2.0))
        };
        let arm_material = {
            let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();
            material_assets.add(Color::srgb_u8(255, 55, 0))
        };

        Self {
            arm_mesh,
            arm_material,
        }
    }
}

impl objects::definition::ObjectBlueprint for HeroAssets {
    type SpawnConfig = Transform;

    fn spawn(cmds: &mut Commands, hero: &Self, transform: Self::SpawnConfig) -> Entity {
        cmds.spawn((
            children![
                (
                    HeroCamera,
                    Camera3d::default(),
                    Transform::from_xyz(0.0, 1.6, 0.0),
                    children![
                        (
                            RayCaster::new(Vec3::ZERO, Dir3::NEG_Z),
                            Transform::default(),
                            DebugTool
                        ),
                        (
                            // arm
                            Mesh3d(hero.arm_mesh.clone()),
                            MeshMaterial3d(hero.arm_material.clone()),
                            Transform::from_xyz(1.0, -1.0, -2.0).looking_at(Vec3::Z, Vec3::Y),
                        ),
                        (
                            // flashlight
                            SpotLight {
                                inner_angle: 0.25,
                                outer_angle: 0.5,
                                range: 200.0,
                                intensity: 5_000_000.0, // readible 5 million
                                shadows_enabled: true,
                                ..default()
                            },
                            Transform::default(),
                        )
                    ],
                ) // (
                  //     HeroBody,
                  //     Mesh3d(mesh.add(Capsule3d::new(0.5, 1.8))),
                  //     MeshMaterial3d(mats.add(Color::srgb_u8(255, 55, 0))),
                  //     Transform::default(),
                  // )
            ],
            transform,
            Visibility::default(),
            Collider::capsule(0.5, 1.8),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            objects::definition::Target,
            WantMove {
                zinput: 0,
                xinput: 0,
                jump: false,
                forward: Vec3::ZERO,
                move_speed: 10.0,
            },
            Hero {
                paused: true,
                sens: Vec2 { x: 0.01, y: 0.01 },
                pitch: 0.0,
                yaw: 0.0,
            },
            ShapeCaster::new(
                Collider::capsule(0.49, 1.79),
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_distance(0.15),
        ))
        .id()
    }
}
