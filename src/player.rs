use std::default;

use bevy::{
    app::{Plugin, PreStartup, Update}, ecs::{component::ComponentId, world::DeferredWorld}, input::{ButtonInput, InputPlugin}, log::info, math::Vec2, prelude::{
        default, in_state, resource_exists, Bundle, Commands, Entity, Event, EventReader, IntoSystemConfigs, KeyCode, Query, Res, Resource, States, World 
    }
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle
};
use bevy_inspector_egui::egui::output;
use bevy_rapier2d::{prelude::{
    ActiveCollisionTypes, ActiveEvents, CharacterLength, Collider, CollisionEvent, ContactForceEvent, KinematicCharacterController, KinematicCharacterControllerOutput, Velocity
}, rapier::prelude::CollisionEventFlags};
use crate::{
    game_flow::GameState,
    unsorted::Id,
};
use super::unsorted::Promise;


#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
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

// #[derive(States)]
// pub enum PlayerState {
//     Idle,
//     Jumping,
//     Falling, etc...
// }


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
            .add_systems(Update, player_collision_handler.run_if(resource_exists::<Player>))
            .add_systems(Update, player_force_contact_handler.run_if(resource_exists::<Player>)) 
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(PreStartup, |world: &mut World| {
                world
                    .register_component_hooks::<Promise<Player>>()
                    .on_add(process_player_promise);
            }) // this could be simplefied with extension trait!?
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
        ActiveCollisionTypes::all(),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.08),
            up: Vec2::Y,
            ..default()
        }
    ));
    entity_commands.remove::<Promise<Player>>();
}

#[derive(Event)]
pub struct PlayerCollision;

pub fn player_collision_handler(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    player: Res<Player>,
) {
    let player_entity = player.entity();
    for collision in collision_events.read() {
        let CollisionEvent::Started(entity_1, entity_2, CollisionEventFlags::SENSOR) = collision else { continue };
        commands.trigger_targets(PlayerCollision, {
            if player_entity == *entity_1 { *entity_2 } else 
            if player_entity == *entity_2 { *entity_1 } else { continue; /*No player collision*/ }
        });
    }
}


// temp test for player killing
pub fn player_force_contact_handler(
    mut collision_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    player: Res<Player>,
) {
    let player_entity = player.entity();
    for collision in collision_events.read() {
        commands.trigger_targets(PlayerCollision, {
            if player_entity == collision.collider1 { collision.collider2 } else 
            if player_entity == collision.collider2 { collision.collider1 } else { continue; /*No player collision*/ }
        });
    }
}


const GRAVITY: f32 = 0.01;
const MOVEMENT_SCALER: f32 = 10.;
fn player_movement(
    mut controller_query: Query<(&mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>)>,
    input: Res<ButtonInput<KeyCode>>,    
) {
    for (mut controller, output) in controller_query.iter_mut() 
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

        // if let Some(output) = output {
        //     if output.grounded && input.pressed(KeyCode::Space) {
        //         movement += Vec2::Y * MOVEMENT_SCALER * 10000.0
        //     }
        // }

        movement.y -= GRAVITY;

        controller.translation = Some(
            controller.translation.unwrap_or_default() + movement.normalize_or_zero() * MOVEMENT_SCALER
        );
    }
}