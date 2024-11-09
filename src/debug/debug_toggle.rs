use bevy::{
    app::{
        Plugin, 
        Update
    }, 
    input::ButtonInput, 
    prelude::{
        KeyCode, 
        Res, 
        ResMut, 
        Resource
    }
};

#[derive(Resource)]
pub struct Debug(bool);

#[allow(dead_code)] 
pub fn debug_active(debug: Res<Debug>) -> bool {debug.0}

fn toggle_debug(
    mut debug: ResMut<Debug>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1)
    {
        debug.0 = !debug.0;
    } 
}


pub struct DebugTogglePlugin;
impl Plugin for DebugTogglePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(Debug(false))
            .add_systems(Update, toggle_debug);
    }
}