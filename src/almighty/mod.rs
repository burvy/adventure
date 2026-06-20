use bevy::prelude::*;

pub struct AlmightyPlugin;

impl Plugin for AlmightyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (logic::update_visibilities, logic::move_all));
        app.add_observer(logic::on_jump_sound);
    }
}

pub mod definition;
pub mod logic;
