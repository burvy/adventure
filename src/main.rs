mod almighty;
mod build;
mod objects;
mod ui;

use crate::build::build_cube;
use crate::objects::cash_register;
use crate::objects::ferris;
use crate::objects::hero;

#[cfg(not(target_arch = "wasm32"))]
use std::env;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::window::WindowResolution;

#[cfg(not(target_arch = "wasm32"))]
use bevy::render::{
    settings::{Backends, WgpuSettings, WgpuSettingsPriority},
    RenderPlugin,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(not(target_arch = "wasm32"))]
use bevy_remote::{
    http::{RemoteHttpPlugin, DEFAULT_PORT},
    RemotePlugin,
};

fn main() -> AppExit {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Adventure".into(),
            resolution: WindowResolution::new(640, 360),
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#adventure-canvas".into()),
            #[cfg(target_arch = "wasm32")]
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    });

    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = default_plugins.set(RenderPlugin {
        render_creation: (WgpuSettings {
            backends: Some(Backends::VULKAN),
            priority: WgpuSettingsPriority::Compatibility,
            ..default()
        })
        .into(),
        ..default()
    });

    let mut app = App::new();
    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
        },
        default_plugins,
    ));
    app.add_plugins(MainPlugin);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let port: u16 = env::args()
            .nth(1)
            .and_then(|arg| arg.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        app.add_plugins(RemotePlugin::default());
        app.add_plugins(RemoteHttpPlugin::default().with_port(port));
    }

    app.run()
}

/// The bare minimum essential functions to run this game.
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(Gravity(Vec3::NEG_Y * 30.0));
        app.add_systems(
            Startup,
            (
                build::build_world::build_lobby,
                build::build_cube::setup_oube,
                ui::crosshair::spawn_crosshair,
            ),
        );
        app.add_observer(ferris::definition::spawn_ferrises);
        app.add_observer(build_cube::spawn_physics_cube);
        app.add_systems(
            Update,
            (
                hero::control::hero_input, // paramount importance
                hero::control::hero_left_click,
                hero::control::read_camera,
                hero::control::update_body.after(hero::control::read_camera),
                hero::control::update_camera.after(hero::control::read_camera),
                ferris::logic::update_ferris, // ferris logic
                ui::input::toggle_pause,
                almighty::logic::update_visibilities,
                almighty::logic::move_all,
            ),
        );
    }
}
