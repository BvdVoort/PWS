mod physics;
mod collision;
mod bundle;
mod promise_procedure;
mod state;

use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bundle::PlayerBundle;
use collision::player_collision_handler;
use physics::PlayerPhysicsPlugin;

use bevy::{app::{First, Plugin, PreStartup}, asset::{AssetServer, Handle}, prelude::{resource_exists, AppExtStates, Commands, Image, IntoSystemConfigs, Res, Resource, World}, sprite::Sprite};

pub use promise_procedure::Player;
pub use collision::PlayerCollision;
use state::PlayerState;

use crate::unsorted::{Promise, PromiseProcedure, Uid};

#[derive(Resource)]
struct ImageHandles {
    pub player: Handle<Image>
}

fn load_sprites(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle = server.load("Character.webp");
    commands.insert_resource(ImageHandles { player: handle });
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .init_state::<PlayerState>()
            .add_systems(PreStartup, load_sprites)
            .add_plugins(PlayerPhysicsPlugin)
            .add_systems(First, player_collision_handler.run_if(resource_exists::<Uid<Player>>))
            .add_systems(PreStartup, |world: &mut World| { 
                world
                    .register_component_hooks::<Promise<Player>>()
                    .on_add(Player::resolve_promise);
                }
            )
            ;
    }
}