mod bundle;
mod character_promise;
mod player_character_controls;
mod physics;
mod input;

use std::time::Duration;

use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bundle::CharacterBundle;
use bevy::{app::{Plugin, PreStartup, PreUpdate, Update}, prelude::{IntoSystemConfigs, World}};
use character_promise::Character;
use input::AxisInputPlugin;
use player_character_controls::{player_gravity, player_jump, player_movement, sync_grounded, CharacterControllerConfig, CustomCharacterData, JumpConfig, JumpTracker, Jumping, Movement, MovementConfig};
use physics::CharacterPhysicsPlugin;
use crate::unsorted::{Promise, PromiseProcedure};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(CharacterControllerConfig {
                movement: Movement {
                    speed_ground: 500.,
                    speed_air: 400.,
                },
                jump_gravity_scale: 0.2,
                fall_gravity_scale: 1.,
            })
            .insert_resource(JumpConfig::new(10., Duration::from_secs_f32(1.)))
            .init_resource::<JumpTracker>()
            .insert_resource(MovementConfig {
                grounded_speed: 5000.,
                air_speed: 0.0,
            })
            .register_ldtk_entity::<CharacterBundle>("Player")
            .add_systems(PreStartup, |world: &mut World| { 
                world
                    .register_component_hooks::<Promise<Character>>()
                    .on_add(Character::resolve_promise);
                }
            )
            // .add_systems(Update, (player_input, update_custom_character_data))
            .add_plugins(AxisInputPlugin)
            .add_plugins(CharacterPhysicsPlugin)
            .add_systems(PreUpdate, sync_grounded)
            .add_systems(Update, (
                player_gravity,
                // player_movement,
                player_jump,
            ))
            .add_plugins(ResourceInspectorPlugin::<CharacterControllerConfig>::new())
            .add_plugins(ResourceInspectorPlugin::<JumpConfig>::new())
            .register_type::<Jumping>()
            .register_type::<CustomCharacterData>()
            .register_type::<MovementConfig>()
            .register_type::<JumpConfig>()
            ;
    }
}