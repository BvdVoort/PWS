use bevy_rapier2d::plugin::RapierConfiguration;
use bevy::{
    app::{
        Plugin, 
        PreStartup
    }, 
    math::Vec2, 
    prelude::ResMut
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {        
        app.add_systems(PreStartup, |mut physics_config: ResMut<RapierConfiguration>| {
            physics_config.gravity = Vec2::NEG_Y
        });
    }
}