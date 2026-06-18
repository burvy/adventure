use bevy::prelude::*;

pub struct PaperPlugin;

impl Plugin for PaperPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<definition::Paper>();
        app.add_systems(Startup, definition::initialize_papers);
    }
}

pub mod definition;
