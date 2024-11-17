use bevy_ecs_ldtk::assets::LdtkProject;
use bevy::{
    asset::Handle, prelude::{
        Commands, DespawnRecursive, Entity, Query, With
    }
};

pub fn despawn_worlds(
    mut commands: Commands,
    levels: Query<Entity, With<Handle<LdtkProject>>>
) {
    for level in levels.iter() {
        commands.add(DespawnRecursive{ entity: level });
    }    
}