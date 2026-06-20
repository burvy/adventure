use crate::objects;
use crate::objects::cash_register;
use crate::objects::ferris;
use bevy::prelude::*;

pub fn on_click(world: &mut World, register: Entity) {
    info!("Light Toggled");
    world.trigger(cash_register::definition::Kaching { register: register });
    let Some(childs) = world
        .get::<Children>(register)
        .map(|children| children.iter().collect::<Vec<_>>())
    else {
        return;
    };
    childs.into_iter().for_each(|child| {
        if let Some(mut visible) = world.get_mut::<objects::definition::Visible>(child) {
            visible.0 = !visible.0;
            return;
        }
    });

    world.trigger(ferris::definition::SpawnFerrisesEvent);
}

// TODO: More generic interface for sounds
/// Kaching sound
/// Mirrors `on_jump_sound` in almighty\logic.rs
pub fn on_kaching(
    trigger: On<cash_register::definition::Kaching>,
    asset_server: Res<AssetServer>,
    mut cmds: Commands,
    transforms: Query<&Transform>,
) {
    info!("Kaching!");
    let kaching_handle = asset_server.load("sounds/kaching.mp3");
    let kaching_event = trigger.event();
    let position = transforms
        .get(kaching_event.register)
        .cloned()
        .unwrap_or_default();
    cmds.spawn((
        AudioPlayer::new(kaching_handle),
        PlaybackSettings::DESPAWN.with_spatial(true),
        position,
    ));
}
