mod input;
mod unsorted;
mod debug;
mod player;
mod physics;

use bevy::{app::{App, Startup}, asset::AssetServer, prelude::{Camera2dBundle, Commands, Res}, DefaultPlugins};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use player::PlayerPlugin;
use unsorted::LDTKEnumTagPluginCustom;


// why does the movement lag?! (disable player movement when in debug mode)
pub fn main() {
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(PlayerPlugin)
        .add_plugins(LDTKEnumTagPluginCustom)
        .add_systems(Startup, setup)
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