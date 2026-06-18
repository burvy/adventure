use bevy::prelude::*;

pub struct FerrisPlugin;

impl Plugin for FerrisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<definition::Ferris>();
        app.add_observer(definition::spawn_ferrises);
        app.add_systems(Update, logic::update_ferris);
    }
}

pub mod definition;
pub mod logic;
