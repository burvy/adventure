use bevy::prelude::*;
pub struct CashRegisterPlugin;

impl Plugin for CashRegisterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<definition::CashRegisterAssets>();
        app.add_observer(logic::on_kaching);
    }
}

pub mod definition;
pub mod logic;
