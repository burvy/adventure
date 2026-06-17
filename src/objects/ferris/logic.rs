use avian3d::prelude::*;
use bevy::prelude::*;

use crate::almighty;
use crate::{almighty::definition::WantMove, objects};

/// Same as update_ferris but uses idiomatic rust
pub fn update_ferris(
    mut ferrises: Query<(&mut Transform, &mut WantMove), Without<objects::definition::Target>>,
    targets: Query<&Transform, With<objects::definition::Target>>,
) {
    ferrises.iter_mut().for_each(|(mut ftf, mut wm)| {
        let Some(target_transform) = targets.iter().min_by(|target_a, target_b| {
            ftf.translation
                .distance_squared(target_a.translation)
                .total_cmp(&ftf.translation.distance_squared(target_b.translation))
        }) else {
            wm.zinput = 0;
            wm.xinput = 0;
            return;
        };
        let mut direction = target_transform.translation - ftf.translation;

        direction.y = 0.0;

        ftf.look_to(direction, Vec3::Y); // look at them
        ftf.rotate_y(std::f32::consts::PI); // rotate 180

        if direction.length_squared() <= 2.0 {
            wm.zinput = 0;
            wm.xinput = 0;
            return;
        }

        wm.forward = direction.normalize();
        wm.zinput = 1;
        wm.xinput = 0;
    });
}

/// Logic for when Ferris is clicked
pub fn click_ferris(entity: Entity, cmds: &mut Commands) {
    info!("Ferris Clicked");
    cmds.entity(entity).despawn();
}
