use std::{ops::Mul, time::Duration};

use bevy::{app::{Plugin, PreUpdate, Startup, Update}, ecs::query, math::Vec2, prelude::{Bundle, Commands, Component, Entity, Event, GamepadButtonType, KeyCode, Local, Query, Res}, reflect::Reflect, time::{Stopwatch, Time}, utils::{default, info}};
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::{ActiveCollisionTypes, ActiveEvents, CharacterCollision, Collider, GravityScale, KinematicCharacterController, KinematicCharacterControllerOutput, Velocity}};
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::{ActionState, GamepadControlAxis, InputMap, KeyboardVirtualAxis, WithAxisProcessingPipelineExt}, Actionlike, InputManagerBundle};

use crate::unsorted::{Promise, PromiseProcedure, BevyPromiseResolver};

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity_with_promise::<Player>("Player")
            .add_plugins(InputManagerPlugin::<InputAction>::default())
            .add_systems(PreUpdate, player_sync_jump_tracker_and_grounded)
            .add_systems(Update, (player_movement, player_colision))
            ;
    }
}



#[derive(Actionlike, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum InputAction {
    #[actionlike(Axis)] Move,
    Jump,
}


#[derive(LdtkEntity, Bundle, Default)]
struct Player {
    promise: Promise<Self>
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Player {
    fn standard_input_map() -> InputManagerBundle<InputAction> {
        InputManagerBundle::with_map(
            InputMap::default()
                // .with_axis(InputAction::Move, KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS) // if you use both arrows and ad you can get double the axis value
                .with_axis(InputAction::Move, KeyboardVirtualAxis::AD)
                .with_axis(InputAction::Move, GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.1))

                .with(InputAction::Jump, KeyCode::Space)
                .with(InputAction::Jump, GamepadButtonType::South)
        )
    }
}

impl PromiseProcedure for Player {
    fn resolve_promise<'w>(mut world: bevy::ecs::world::DeferredWorld<'w>, entity: bevy::prelude::Entity, component_id: bevy::ecs::component::ComponentId) {
        world
            .commands()
            .entity(entity)
            .insert((
                Self::standard_input_map(),
                KinematicCharacterController::default(),
                Collider::capsule_y(4., 4.),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                JumpTracker::default(),
                Velocity::default(),
                GravityScale::default(),
            ));
        world.commands().entity(entity).remove_by_id(component_id);
    }
}

const MOVEMENT_SCALER: f32 = 200.0;
const TEMP_GRAVITY_SCALER: f32 = 55.0;

fn player_movement(
    time: Res<Time>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&ActionState<InputAction>, &mut JumpTracker, &mut KinematicCharacterController, &mut Velocity, &mut GravityScale)>,
) {
    for (action, mut jump, mut controller, mut velocity, mut gravity_scale) in query.iter_mut() {
        let horizontal_movement = action
            .axis_data(&InputAction::Move)
            .unwrap_or(&default())
            .value 
            .mul(MOVEMENT_SCALER) 
            .mul(time.delta_seconds());

        if action.pressed(&InputAction::Jump) {
            jump.try_jump(|| {
                *velocity = Velocity::linear(Vec2::Y * 100.0);        
                gravity_scale.0 = 0.3;
            });
        } 
        else {
            gravity_scale.0 = 1.0;
        }
        *velocity = Velocity::linear(rapier_config.gravity * gravity_scale.0 * time.delta_seconds() * TEMP_GRAVITY_SCALER + velocity.linvel);
        controller.translation = Some(Vec2::new(horizontal_movement, velocity.linvel.y * time.delta_seconds()))
    }
}

#[derive(Component, Reflect, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
enum Jump {
    #[default]
    Impossible,
    Possible,
    Started,
    Performing,
}

impl Jump {
    pub fn can_jump(&self) -> bool { *self == Self::Possible }
    pub fn try_jump(&mut self, jump_action: impl FnOnce()) {
        if self.can_jump() { unsafe { self.jump(jump_action) }; }
    }

    pub unsafe fn jump(&mut self, jump_action: impl FnOnce()) {
        jump_action();
        *self = Self::Started;
    }
}

#[derive(Component, Reflect, Debug, Default, PartialEq, Clone)]
struct JumpTracker {
    time_since_last: Stopwatch,
    state: Jump,
}

impl JumpTracker {
    const COYOTE_TIME: Duration = Duration::from_millis(100);

    fn can_jump(&self) -> bool { self.state.can_jump() || (self.state == Jump::Impossible && self.time_since_last.elapsed() < Self::COYOTE_TIME) }
    fn try_jump(&mut self, jump_action: impl FnOnce()) { 
        if self.can_jump() { unsafe { self.jump(jump_action); } } 
    }

    pub unsafe fn jump(&mut self, jump_action: impl FnOnce()) {
        unsafe { self.state.jump(jump_action); }
        self.time_since_last.reset();
    }
}

fn player_sync_jump_tracker_and_grounded(
    mut query: Query<(&mut JumpTracker, &KinematicCharacterControllerOutput)>
) {
    for (mut jump, check) in query.iter_mut() {
        jump.state = match jump.state {
            Jump::Performing |
            Jump::Impossible => if check.grounded { Jump::Possible } else { continue; },
            Jump::Started => if !check.grounded { Jump::Performing } else { continue; }, // #! NOTE: if jump won't get off the ground this wil fail!
            Jump::Possible => continue,
        }
    }
}

#[derive(Event)]
pub struct CollidedWithCharacter(Entity);
impl CollidedWithCharacter {
    pub fn entity(&self) -> Entity { self.0 }
}

fn player_colision(
    mut commands: Commands,
    mut query: Query<(Entity, &KinematicCharacterControllerOutput)>
) {
    for (character_entity, controller_output) in query.iter_mut() {
        for colision in controller_output.collisions.clone() {
            if character_entity == colision.entity {
                info("colision is characgter");
            }
            commands.trigger_targets(CollidedWithCharacter(character_entity), colision.entity); //let others handle damage etc
        }
    }
}