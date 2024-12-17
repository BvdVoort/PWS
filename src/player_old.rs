use std::{default, marker::PhantomData, ops::{Add, AddAssign}, time::Duration};

use bevy::{
    app::{First, Last, Plugin, PreStartup, PreUpdate, Update}, ecs::{component::{ComponentHook, ComponentId}, schedule::{SystemConfig, SystemConfigs}, world::DeferredWorld}, input::{keyboard::Key, ButtonInput, InputPlugin}, log::info, math::Vec2, prelude::{
        default, in_state, resource_exists, AppExtStates, Bundle, Commands, Component, Entity, Event, EventReader, IntoSystem, IntoSystemConfigs, KeyCode, NextState, OnEnter, Query, Res, ResMut, Resource, State, States, System, World 
    }, time::Stopwatch
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle
};
use bevy_inspector_egui::egui::output;
use bevy_rapier2d::{plugin::{RapierConfiguration, RapierContext}, prelude::{
    ActiveCollisionTypes, ActiveEvents, CharacterLength, Collider, CollisionEvent, ContactForceEvent, GravityScale, KinematicCharacterController, KinematicCharacterControllerOutput, MassProperties, RigidBody, Velocity
}, rapier::prelude::CollisionEventFlags};
use crate::{
    game_flow::GameState,
    unsorted::Id,
};
use super::unsorted::Promise;

const GRAVITY: f32 = 1.;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player: Promise<Player>,
    gravity_scale: GravityScale,
    acceleration: Accelleration,
    velocity: Velocity,
}

#[derive(Resource)]
pub struct Player(Id<Player>);
impl Player {
    pub fn entity(&self) -> Entity {
        self.0.entity()
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    // Running,
    Jumping,
    Falling,
}

#[derive(Resource, Default)]
struct LastJumped(Stopwatch);

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
            .init_resource::<LastJumped>()
            .init_state::<PlayerState>()    
            // .add_schedule(First, ) // at first get input then pre update apply
            .add_systems(PreUpdate, apply_acceleration.before(apply_velocity))
            .add_systems(First, apply_gravity_to_character)
            .add_systems(OnEnter(PlayerState::Jumping), |mut last_jumped: ResMut<LastJumped>| { last_jumped.0.reset() })
            
            .add_systems(Update, player_collision_handler.run_if(resource_exists::<Player>))
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(PreStartup, (
                |world: &mut World| { world
                    .register_component_hooks::<Promise<Player>>()
                    .on_add(process_player_promise);
            }))            
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


// should use custom forces, acceleration and velocity
fn jump(
    mut controller_query: Query<(&mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>)>,
    input: Res<ButtonInput<KeyCode>>,
    last_jumped: Res<LastJumped>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for (mut controller, output) in controller_query.iter_mut() {
        let grounded = if let Some(output) = output { output.grounded } else { false };
        let jump_button_pressed = input.any_pressed([KeyCode::Space]);
        match player_state.get() {
            PlayerState::Idle | PlayerState::Walking => {
                if jump_button_pressed {
                    next_player_state.set(PlayerState::Jumping);
                    controller.translation.add_assign(Vec2::Y * 100.);  // add jump force
                }
            },
            PlayerState::Jumping => {
                if jump_button_pressed && last_jumped.0.elapsed() < Duration::from_secs_f32(5.) {
                    controller.translation.add_assign(Vec2::Y * 100.); // add jump force
                }
                else {
                    // set gravity scale to a larger value
                    controller.translation.add_assign(Vec2::NEG_Y * 50.);
                }
            },
            PlayerState::Falling => {
                // add Gravity scale 1
            },
        }
    }
}




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
        // if player_state
        // if let Some(output) = output {
        //     if output.grounded && input.pressed(KeyCode::Space) {
        //         movement += Vec2::Y * MOVEMENT_SCALER * 10000.0
        //     }
        // }

        controller.translation = Some(
            controller.translation.unwrap_or_default() + movement.normalize_or_zero() * MOVEMENT_SCALER
        );
    }
}