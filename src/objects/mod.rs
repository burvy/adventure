use bevy::prelude::*;

pub mod cash_register;
pub mod definition;
pub mod ferris;
pub mod hero;
pub mod paper;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cash_register::CashRegisterPlugin,
            ferris::FerrisPlugin,
            hero::HeroPlugin,
            paper::PaperPlugin,
        ));
    }
}
