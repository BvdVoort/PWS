mod unsorted;
mod debug;
mod player;
mod physics;
mod game_flow;
mod font_handing;
mod enemies;
mod collision;
mod finish;

use bevy::{app::{App, Startup, Update}, asset::AssetServer, math::Vec3, prelude::{AppExtStates, Camera, Camera2dBundle, Commands, OnEnter, OnExit, Query, Res, ResMut, Transform, With}, utils::default, DefaultPlugins};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use font_handing::{FontHandles, FontPlugin};
use game_flow::GameState;
use player::PlayerPlugin;
use unsorted::LDTKEnumTagPluginCustom;

use unsorted::ldtk_level_handler;

// why does (debug) the movement lag?! (disable player movement when in debug mode)
pub fn main() {
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .init_state::<game_flow::GameState>()

        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10000.))
        .add_plugins(PlayerPlugin)
        .add_plugins(LDTKEnumTagPluginCustom)

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        .add_plugins(FontPlugin)
        .add_systems(OnEnter(GameState::Defeated), spawn_defeat_text) // should be a defeated event
        .add_systems(OnEnter(GameState::Completed), spawn_complete_text) // should be a completed event
        
        .add_plugins(enemies::EnemyPlugin)

        .add_plugins(finish::FinishPlugin)

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
    camera.transform.translation.x += 1280.0 / 4.0 + 100.;
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
        game_state.set(GameState::Defeated);
    }
    else if input.pressed(bevy::prelude::KeyCode::KeyV) {
        game_state.set(GameState::Completed);
    }
}

use bevy::text::{
    Text2dBundle,
    Text,
    TextStyle,
};

fn spawn_defeat_text(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let text_style = TextStyle {
        font: fonts.default_font(),
        font_size: 60.0,
        ..default()
    };

    let mut text = Text2dBundle {
        text: Text::from_section("You died!", text_style.clone()),
        ..default()
    };

    camera.single_mut().translation = Vec3::ZERO;

    commands.spawn(text);
}

fn spawn_complete_text(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let text_style = TextStyle {
        font: fonts.default_font(),
        font_size: 60.0,
        ..default()
    };

    let mut text = Text2dBundle {
        text: Text::from_section("You win!", text_style.clone()),
        ..default()
    };

    camera.single_mut().translation = Vec3::ZERO;

    commands.spawn(text);
}