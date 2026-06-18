use bevy::prelude::*;

pub mod build_cube;
pub mod build_world;
pub mod layout;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<build_cube::Oube>();
        app.init_resource::<layout::LobbyLayout>();
        app.add_systems(Startup, build_world::build_lobby);
        app.add_observer(build_cube::spawn_physics_cube);
    }
}
