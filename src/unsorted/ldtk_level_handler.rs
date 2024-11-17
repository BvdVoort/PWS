use bevy_ecs_ldtk::assets::LdtkProject;
use bevy::{
    asset::Handle, log::info, prelude::{
        Commands, 
        Entity, 
        Query, 
        With
    }
};

pub fn despawn_worlds(
    mut commands: Commands,
    levels: Query<Entity, With<Handle<LdtkProject>>>
) {
    for level in levels.iter() {
        commands.entity(level).despawn();
    }    
}