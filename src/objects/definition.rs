use bevy::prelude::*;

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
