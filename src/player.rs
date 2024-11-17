use std::default;

use bevy::{
    app::{Plugin, PreStartup, Update}, ecs::{component::ComponentId, world::DeferredWorld}, input::{ButtonInput, InputPlugin}, math::Vec2, prelude::{
        default, in_state, Bundle, Entity, IntoSystemConfigs, KeyCode, Query, Res, Resource, World 
    }
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle
};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, CharacterLength, Collider, KinematicCharacterController, KinematicCharacterControllerOutput, Velocity 
};
use crate::{
    game_flow::GameState,
    unsorted::Id,
};
use super::unsorted::Promise;


#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player: Promise<Player>,

    velocity: Velocity,
}

#[derive(Resource)]
pub struct Player(Id<Player>);
impl Player {
    pub fn entity(&self) -> Entity {
        self.0.entity()
    }
}


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<InputPlugin>()
        {
            app.add_plugins(InputPlugin);
        }
        
        app
            .register_ldtk_entity::<PlayerBundle>("Player")    
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(PreStartup, |world: &mut World| {
                world
                    .register_component_hooks::<Promise<Player>>()
                    .on_add(process_player_promise);
            })
            ;
    }
}

fn process_player_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let mut commands = world.commands();
    commands.insert_resource(Player(entity.into()));
    let mut entity_commands = commands.entity(entity);
    entity_commands.insert((
        Collider::capsule_y(4., 4.),
        ActiveEvents::COLLISION_EVENTS,
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.08),
            up: Vec2::Y,
            ..default()
        }
    ));
    entity_commands.remove::<Promise<Player>>();
}

const GRAVITY: f32 = 0.1;
const MOVEMENT_SCALER: f32 = 10.;
fn player_movement(
    mut controllers: Query<(&mut KinematicCharacterController)>,
    input: Res<ButtonInput<KeyCode>>,    
) {
    for mut controller in controllers.iter_mut() 
    {
        let mut movement = Vec2::ZERO;
        if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            movement += Vec2::NEG_X;
        }
        if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            movement += Vec2::X;
        }
        if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            movement += Vec2::Y;
        }
        if input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            movement += Vec2::NEG_Y;
        }

        if input.pressed(KeyCode::Space) {
            movement += Vec2::Y * MOVEMENT_SCALER
        }

        movement.y -= GRAVITY;
        controller.translation = Some(
            controller.translation.unwrap_or_default() + movement.normalize_or_zero() * MOVEMENT_SCALER
        );
    }
}