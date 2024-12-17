use bevy::{input::ButtonInput, prelude::{KeyCode, Query, Res}};
use super::Acceleration;


pub fn apply_walk_movement(
    mut acceleration_query: Query<&mut Acceleration>,
    input: Res<ButtonInput<KeyCode>>,    
) {
    let mut x_axis = Acceleration::ZERO;
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { x_axis += Acceleration::LEFT * 100. }
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { x_axis += Acceleration::RIGHT * 100. }

    for mut acceleration in acceleration_query.iter_mut() {
        *acceleration += x_axis; // #? TODO: set a acceleration maximum
    }
}