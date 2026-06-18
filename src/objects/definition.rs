use bevy::prelude::*;

/// Add this struct onto an object if it is interactable
/// The value of the field is a function defining the
/// logic that should run when this object is clicked
/// by the hero
#[derive(Component, Clone, Copy)]
pub struct Interactable {
    pub on_click: fn(&mut World, Entity),
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Visible(pub bool);

pub trait ObjectBlueprint: Resource {
    type SpawnConfig;

    fn spawn(cmds: &mut Commands, blueprint: &Self, config: Self::SpawnConfig) -> Entity;
}

pub fn spawn_object<T: ObjectBlueprint>(
    cmds: &mut Commands,
    blueprint: &T,
    config: T::SpawnConfig,
) -> Entity {
    T::spawn(cmds, blueprint, config)
}

pub fn spawn_many<T: ObjectBlueprint>(
    cmds: &mut Commands,
    blueprint: &T,
    configs: impl IntoIterator<Item = T::SpawnConfig>,
) {
    for config in configs {
        spawn_object::<T>(cmds, blueprint, config);
    }
}
