use bevy::prelude::*;

#[derive(Component)]
pub struct FixPerspectiveSubject {
    // The original transform from which the entity is getting corrected to
    pub original_transform: Transform,
}

/// Marker component for the camera that is target for perspective fixing
#[derive(Component)]
pub struct FixPerspectiveTarget;


/// Place all hexagons along a circle and face them at the camera
pub fn fix_perspective_system(
    mut target_query: Query<(&Transform, &FixPerspectiveTarget), Without<FixPerspectiveSubject>>,
    mut subject_query: Query<(&mut Transform, &FixPerspectiveSubject), Without<FixPerspectiveTarget>>,
) {
    for (target_transform, _) in target_query.iter_mut() {
        // Physics hexagons have their origin at z = 0, meaning that the radius is equal to
        // camera's z position
        let radius = target_transform.translation.z;
        for (mut subject_transform, fix_perspective_subject) in subject_query.iter_mut() {
            // Calculate difference vector between original translation and target position
            let difference = target_transform.translation - fix_perspective_subject.original_transform.translation;
            // Set translation of subject onto circle
            let scaled_translation = difference.normalize() * (difference.length() - radius);
            let _ = subject_transform.with_translation(fix_perspective_subject.original_transform.translation + scaled_translation);
            // Rotate towards camera
            subject_transform.look_at(target_transform.translation, Vec3::Y);
        }
    }
}