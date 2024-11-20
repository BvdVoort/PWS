mod physics;
mod collision;
mod bundle;
mod promise_procedure;

use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bundle::PlayerBundle;
use collision::player_collision_handler;
use physics::PlayerPhysicsPlugin;

use bevy::{app::{First, Plugin, PreStartup}, prelude::{resource_exists, IntoSystemConfigs, World}};

pub use promise_procedure::Player;
pub use collision::PlayerCollision;

use crate::unsorted::{Promise, PromiseProcedure, Uid};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins(PlayerPhysicsPlugin)
            .add_systems(First, player_collision_handler.run_if(resource_exists::<Uid<Player>>))
            .add_systems(PreStartup, |world: &mut World| { 
                world
                    .register_component_hooks::<Promise<Player>>()
                    .on_add(Player::invoke);
                }
            );
    }
}