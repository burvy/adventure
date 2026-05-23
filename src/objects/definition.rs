use bevy::prelude::*;

#[derive(Component)]
/// List of different interaction enumerations.
pub enum Thing {
    CashRegister,
    Ferris,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct ForwardCast;

#[derive(Component)]
pub struct Visible(pub bool);
