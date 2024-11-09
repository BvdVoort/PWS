use bevy::{
    app::{
        Plugin, 
        Update
    }, 
    prelude::{
        IntoSystemSetConfigs, 
        SystemSet
    }
};

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Debug;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Input;


pub(super) struct DebugSetsPlugin;
impl Plugin for DebugSetsPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .configure_sets(Update, Debug.run_if(super::debug_toggle::debug_active))
            .configure_sets(Update, Input.in_set(Debug));
    }
}