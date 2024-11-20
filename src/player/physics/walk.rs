use bevy::{input::ButtonInput, prelude::{KeyCode, Query, Res}};
use super::Acceleration;

pub fn apply_walk_movement(
    mut acceleration_query: Query<&mut Acceleration>,
    input: Res<ButtonInput<KeyCode>>,    
) {
    let mut x_axis = Acceleration::ZERO;
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { x_axis -= Acceleration::LEFT }
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { x_axis += Acceleration::RIGHT }
    
    for mut acceleration in acceleration_query.iter_mut() {
        *acceleration += x_axis; // set a acceleration max
    }
}