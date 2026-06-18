use bevy::prelude::*;
use bevy::window::*;

use crate::objects::hero;

use crate::KeyCode::Escape;

/// Pause and unpause state
pub fn toggle_pause(
    cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut hero: Single<&mut hero::definition::Hero, With<hero::definition::Hero>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(Escape) {
        hero.paused = !hero.paused;
    }
    match hero.paused {
        true => pause(cursor),
        false => play(cursor, window),
    }
}
/// Actions to do if paused.
fn pause(mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    cursor.grab_mode = CursorGrabMode::None;
    cursor.visible = true;
}
/// Actions to do if unpaused
fn play(
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    let center = Some(window_center(&window));

    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
    window.set_cursor_position(center);
}

/// Returns the center of the window.
fn window_center(window: &Window) -> Vec2 {
    Vec2::new(window.width() / 2.0, window.height() / 2.0)
}
