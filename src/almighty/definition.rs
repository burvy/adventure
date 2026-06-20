use bevy::prelude::*;

/// A struct to indicate where an entity wants to move.
#[derive(Component)]
pub struct WantMove {
    pub zinput: i8,
    pub xinput: i8,
    pub jump: bool,
    pub forward: Vec3,
    pub move_speed: f32,
}

#[derive(Event)]
pub struct Boing {
    pub jumper: Entity,
}
