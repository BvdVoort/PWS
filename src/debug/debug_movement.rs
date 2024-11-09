use bevy::{
    input::ButtonInput, 
    math::Vec3, 
    prelude::{
        Camera, 
        KeyCode, 
        OrthographicProjection, 
        Query, 
        Res, 
        Transform, 
        With
    }, 
    time::Time
};

// update debug movement to be toggled and 
// replace the camera attached to the player and return to attached when toggled back
// when toggle to debug start from current cameras position
// this becomes a plugin
#[cfg(feature = "debug")]
#[allow(dead_code)]
pub(super) fn free_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.01 {
            ortho.scale = 0.01;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 250. * ortho.scale;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}