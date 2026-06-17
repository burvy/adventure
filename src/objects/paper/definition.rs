use avian3d::prelude::*;
use bevy::prelude::*;

use crate::objects;

#[derive(Resource)]
struct Paper {
    pub mesh: Handle<Mesh>,
    pub mats: Handle<StandardMaterial>,
}

pub fn setup_paper(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    cmds.insert_resource(Paper {
        mesh: mesh.add(Cuboid::new(2.0, 4.0, 0.5)),
        mats: mats.add(Color::srgb_u8(255, 255, 255)),
    })
}

// Paper builder
pub fn build_papers(cmds: &mut Commands, paper: &Paper, pos: Vec3) {
    todo!("Make paper collectible")
}
