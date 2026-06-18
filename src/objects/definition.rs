use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Interactable {
    pub on_click: fn(&mut World, Entity),
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Visible(pub bool);
