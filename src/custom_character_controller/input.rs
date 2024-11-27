use bevy::{app::{Plugin, PreUpdate}, input::ButtonInput, log::info, prelude::{KeyCode, Res, ResMut, Resource}, reflect::Reflect, utils::tracing::{field::{Field, Visit}, Value}};


pub struct AxisInputPlugin;
impl Plugin for AxisInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<MovementAxis>()
            .register_type::<MovementAxis>()
            .add_systems(PreUpdate, update_movement_axes);
    }
}

#[derive(Resource, Reflect, Default, Clone)]
pub struct MovementAxis {
    x: f32
}

impl MovementAxis {
    pub fn horizontal(&self) -> f32 { self.x }
}

pub fn update_movement_axes(
    input: Res<ButtonInput<KeyCode>>,
    mut axis: ResMut<MovementAxis>
) {
    *axis = MovementAxis::default();
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { axis.x -= 1.0; } 
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { axis.x += 1.0; } 
}