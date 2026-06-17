use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Paper {
    pub mesh: Handle<Mesh>,
    pub mats: Handle<StandardMaterial>,
    pub size: Vec3,
}

pub fn setup_paper(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    cmds.insert_resource(Paper {
        mesh: mesh.add(Cuboid::new(2.0, 4.0, 0.5)),
        mats: mats.add(Color::srgb_u8(255, 255, 255)),
        size: Vec3::new(2.0, 4.0, 0.5),
    })
}

// Paper builder
pub fn build_papers(cmds: &mut Commands, paper: &Paper, pos: Vec3) {
    cmds.spawn((
        Mesh3d(paper.mesh.clone()),
        MeshMaterial3d(paper.mats.clone()),
        Transform::from_xyz(pos.x, pos.y + 2.5, pos.z),
        Collider::cuboid(paper.size.x, paper.size.y, paper.size.z),
        RigidBody::Dynamic,
    ));
}

pub fn initialize_papers(mut cmds: Commands, paper: Res<Paper>) {
    (0..50).for_each(|_| {
        build_papers(
            &mut cmds,
            &paper,
            Vec3 {
                x: rand::random_range(-25.0..25.0),
                y: 0.0,
                z: rand::random_range(-25.0..25.0),
            },
        )
    });
}
