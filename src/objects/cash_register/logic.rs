use crate::objects;
use crate::objects::ferris;
use bevy::prelude::*;

pub fn on_click(world: &mut World, register: Entity) {
    info!("Light Toggled");

    let Some(childs) = world
        .get::<Children>(register)
        .map(|children| children.iter().collect::<Vec<_>>())
    else {
        return;
    };

    for child in childs {
        if let Some(mut visible) = world.get_mut::<objects::definition::Visible>(child) {
            visible.0 = !visible.0;
            break;
        }
    }

    world.trigger(ferris::definition::SpawnFerrisesEvent);
}
