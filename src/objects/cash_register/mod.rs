use bevy::prelude::*;

pub struct CashRegisterPlugin;

impl Plugin for CashRegisterPlugin {
    fn build(&self, _app: &mut App) {}
}

pub mod definition;
pub mod logic;
