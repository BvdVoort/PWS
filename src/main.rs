mod input;
mod unsorted;
mod debug;
mod player;
mod physics;

use bevy::{app::{App, Startup, Update}, asset::AssetServer, prelude::{Added, Camera2dBundle, Commands, Entity, IntoSystemConfigs, Query, Res}, DefaultPlugins};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection, TileEnumTags};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::Collider, render::RapierDebugRenderPlugin};
use player::PlayerPlugin;


// why does the movement lag?! (disable player movement when in debug mode)
pub fn main() {
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, proces_tile_tags)
        .insert_resource(LevelSelection::index(0));                        
    
    #[cfg(feature = "debug")]
    app
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(debug::DebugPlugin);
        
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..Default::default()
    });
}


// needs to move!!
// #! do something with tags (need a similar function for metadata)
// #! need someting to keep track of metadata (and enumtags)!
fn proces_tile_tags(
    mut commands: Commands,
    tagged_tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>
) {
    for (entity, selected) in tagged_tiles.iter()
    {
        let Some(enum_id) = selected.source_enum_uid else {
            continue; // #! expected enum id?!
        };

        match enum_id {
            117 => // Collision enum 
            {
                for tag in selected.tags.iter() { match tag as &str {
                    "Solid" => { 
                        commands.entity(entity).insert((
                            Collider::cuboid(8., 8.),
                        )); 
                    }
                    _ => panic!("Unkown tag from enum{enum_id} attached to entity{entity}! tag: {tag}")
                }}
            }
            _ => panic!("Tag with unknown enum attached to {entity}! Id: {enum_id}; Tags: {:?}", selected.tags)    
        }

        commands.entity(entity).remove::<TileEnumTags>();
    }
}