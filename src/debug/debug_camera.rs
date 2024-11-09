use bevy::{app::{Plugin, Startup}, prelude::{Camera2dBundle, Commands, Component, IntoSystemConfigs}};

use super::Debug;

#[derive(Component)]
pub struct DebugCamera;

fn spawn_debug_camera(
    mut command: Commands
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.camera.is_active = false;
    
    command.spawn((
        DebugCamera,
        camera_bundle,
    ));
}

fn debug_enabled(){ println!("Debug enabled (toggle f1).") }


pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(Startup, debug_enabled.in_set(Debug))
            ;
    }
}