use bevy::{
    app::{
        Plugin, 
        Update
    }, 
    prelude::SystemSet
};

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardInput;

#[cfg(feature = "debug")]
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct DebugInput;

pub struct InputPlugin;
impl Plugin for InputPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(Update, (StandardInput, #[cfg(feature = "debug")]DebugInput));
    }
}

