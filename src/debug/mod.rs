mod debug_sets;
mod debug_toggle;
mod debug_movement;
mod debug_camera;


// set structure
// Debug
//      Input   (in debug set)
#[allow(unused_imports)]
pub use debug_sets::{
    Debug, 
        Input,
};

// default plugin definition
use debug_sets::DebugSetsPlugin;
use debug_toggle::DebugTogglePlugin;
use bevy::{
    app::{
        Plugin, 
        Update
    }, 
    prelude::IntoSystemConfigs
};

pub struct DebugPlugin;
impl Plugin for DebugPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {        
        app
            // .add_systems(Startup, spawn_debug_camera)// make bundle
            .add_systems(Update, debug_movement::free_movement.in_set(Input))
            .add_plugins((DebugSetsPlugin, DebugTogglePlugin));

        // replace the camera attached to the player and return to attached when toggled back
        // when toggle to debug start from current cameras position
    }
}