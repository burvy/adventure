use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{almighty, objects};

const TURN_SPEED: f32 = 1.0;

/// Same as update_ferris but uses idiomatic rust
pub fn update_ferris(
    time: Res<Time>,
    mut ferrises: Query<
        (
            &mut Transform,
            &mut almighty::definition::WantMove,
            &ShapeHits,
        ),
        Without<objects::definition::Target>,
    >,
    targets: Query<&Transform, With<objects::definition::Target>>,
) {
    ferrises.iter_mut().for_each(|(mut ftf, mut wm, sh)| {
        let Some(ttf) = targets.iter().min_by(|target_a, target_b| {
            ftf.translation
                .distance_squared(target_a.translation)
                .total_cmp(&ftf.translation.distance_squared(target_b.translation))
        }) else {
            wm.zinput = 0;
            wm.xinput = 0;
            return;
        };

        let mut pdir = ttf.translation - ftf.translation;
        pdir.y = 0.0;

        if pdir.length_squared() <= 16.0 {
            wm.zinput = 0;
            wm.xinput = 0;
            // TODO: detect grounded before jumping
            if almighty::logic::validate_jump(sh) {
                wm.jump = true;
            }
            return;
        }

        let pdir = pdir.normalize();

        // flip bc bevy models are flipped
        let target_rotation = (Quat::from_rotation_y(std::f32::consts::PI)
            * Transform::default().looking_to(pdir, Vec3::Y).rotation)
            .normalize();

        let rot_prog = (TURN_SPEED * time.delta_secs()).clamp(0.0, 1.0);

        ftf.rotation = ftf.rotation.slerp(target_rotation, rot_prog);

        // also flipped bc bevy models are flipped
        wm.forward = -ftf.forward().as_vec3();
        wm.zinput = 1;
        wm.xinput = 0;
    });
}

/// Logic for when Ferris is clicked
pub fn click_ferris(entity: Entity, cmds: &mut Commands) {
    info!("Ferris Clicked");
    cmds.entity(entity).despawn();
}
