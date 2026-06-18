use bevy::prelude::*;

pub struct CashRegisterPlugin;

impl Plugin for CashRegisterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<definition::CashRegisterAssets>();
    }
}

pub mod definition;
pub mod logic;
