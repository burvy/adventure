use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty;
use crate::{almighty::definition::WantMove, objects};

/// Move Ferrises
pub fn update_ferris(
    mut ferrises: Query<(&mut Transform, &mut WantMove), Without<objects::definition::Target>>,
    targets: Query<&Transform, With<objects::definition::Target>>,
) {
    // for each ferris
    for (mut ferris_transform, mut want_move) in &mut ferrises {
        let Some(target_transform) = targets.iter().min_by(|target_a, target_b| {
            ferris_transform
                .translation
                .distance_squared(target_a.translation)
                .total_cmp(
                    &ferris_transform
                        .translation
                        .distance_squared(target_b.translation),
                )
        }) else {
            want_move.zinput = 0;
            want_move.xinput = 0;
            continue;
        };
        let mut direction = target_transform.translation - ferris_transform.translation;

        direction.y = 0.0;

        ferris_transform.look_to(direction, Vec3::Y); // look at them
        ferris_transform.rotate_y(std::f32::consts::PI); // rotate 180

        if direction.length_squared() <= 2.0 {
            want_move.zinput = 0;
            want_move.xinput = 0;
            continue;
        }

        want_move.forward = direction.normalize();
        want_move.zinput = 1;
        want_move.xinput = 0;
    }
}

/// Logic for when Ferris is clicked
pub fn click_ferris(entity: Entity, cmds: &mut Commands) {
    info!("Ferris Clicked");
    cmds.entity(entity).despawn();
}
