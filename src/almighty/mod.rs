use bevy::prelude::*;

pub struct AlmightyPlugin;

impl Plugin for AlmightyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (logic::update_visibilities, logic::move_all));
    }
}

pub mod definition;
pub mod logic;
