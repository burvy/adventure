use bevy::prelude::*;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<definition::HeroAssets>();
        app.add_systems(
            Update,
            (
                control::hero_input,
                control::hero_left_click,
                control::read_camera,
                control::update_body.after(control::read_camera),
                control::update_camera.after(control::read_camera),
            ),
        );
        app.add_observer(logic::add_score);
    }
}

pub mod control;
pub mod definition;
pub mod logic;
