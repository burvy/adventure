use bevy::prelude::*;

use crate::build::layout::{LobbyLayout, spawn_layout, spawn_static_scenes};
use crate::objects::{cash_register, hero};

/// Construct the default environment to spawn in.
pub fn build_lobby(
    mut cmds: Commands,
    hero_assets: Res<hero::definition::HeroAssets>,
    cash_register_assets: Res<cash_register::definition::CashRegisterAssets>,
    lobby_layout: Res<LobbyLayout>,
    asset_server: Res<AssetServer>,
) {
    spawn_layout::<hero::definition::HeroAssets>(
        &mut cmds,
        &hero_assets,
        lobby_layout.hero_spawns(),
    );
    spawn_layout::<cash_register::definition::CashRegisterAssets>(
        &mut cmds,
        &cash_register_assets,
        lobby_layout.cash_register_spawns(),
    );
    spawn_static_scenes(&mut cmds, &asset_server, lobby_layout.static_scenes());
}
