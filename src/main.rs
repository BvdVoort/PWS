mod unsorted;
mod debug;
mod player;
mod physics;
mod game_flow;

use bevy::{app::{App, PostStartup, Startup, Update}, asset::AssetServer, prelude::{AppExtStates, Camera2dBundle, Commands, OnExit, Res, ResMut}, DefaultPlugins};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use game_flow::GameState;
use player::PlayerPlugin;
use unsorted::LDTKEnumTagPluginCustom;

use unsorted::ldtk_level_handler;

// why does the movement lag?! (disable player movement when in debug mode)
pub fn main() {
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .init_state::<game_flow::GameState>()

        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(PlayerPlugin)
        .add_plugins(LDTKEnumTagPluginCustom)

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        // temp
        .add_systems(Update, kill_or_complete_on_keypress)

        .add_systems(OnExit(GameState::Playing), ldtk_level_handler::despawn_worlds) 
        ;

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

// temp test junk
fn kill_or_complete_on_keypress(
    input: Res<bevy::input::ButtonInput<bevy::prelude::KeyCode>>,
    mut game_state: ResMut<bevy::prelude::NextState<GameState>>  
) {
    if input.pressed(bevy::prelude::KeyCode::KeyK) {
        game_state.set(GameState::Dead);
    }
    else if input.pressed(bevy::prelude::KeyCode::KeyV) {
        game_state.set(GameState::Completed);
    }
}