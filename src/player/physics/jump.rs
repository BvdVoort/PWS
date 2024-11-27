use std::{fmt::Debug, time::Duration};

use bevy::{app::{First, Last, Plugin, Update}, core::FrameCount, ecs::component::Tick, input::ButtonInput, log::info, prelude::{resource_exists, Component, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, State, With}, reflect::Reflect, time::{Stopwatch, Time}};
use bevy_rapier2d::prelude::{GravityScale, KinematicCharacterControllerOutput};
use crate::player::{promise_procedure::PlayerTag, state::{self, PlayerState}};

use super::Acceleration;

const FALLING_GRAVITY_MULTIPLIER: f32 = 5.0;

pub struct JumpPlugin;
impl Plugin for JumpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<JumpForce>()
            .init_resource::<JumpedLast>()
            .register_type::<JumpedLast>()
            .add_systems(Update, (
                (
                    align_grounded_check_with_player_state,
                    check_jump,
                )
                .chain()
                .run_if(resource_exists::<NextState<PlayerState>>),
                JumpedLast::tick,
            ))
            .add_systems(OnEnter(PlayerState::Jumping), (JumpForce::apply, JumpedLast::register_jump))
            // .add_systems(OnEnter(PlayerState::Falling), |mut query: Query<&mut GravityScale, With<PlayerTag>>|{
            //     for mut scale in query.iter_mut() { scale.0 *= FALLING_GRAVITY_MULTIPLIER; } // floating point error! solution?: scale.0 = (2.0 * scale.0).round();
            // })
            // .add_systems(OnExit(PlayerState::Falling), |mut query: Query<&mut GravityScale, With<PlayerTag>>|{
            //     for mut scale in query.iter_mut() { scale.0 /= FALLING_GRAVITY_MULTIPLIER; } // floating point error! solution?: scale.0 = (2.0 / scale.0).round();
            // })
            .add_systems(First, print_resource::<State<PlayerState>>)
            ;
    }
}

fn print_resource<R: Resource + Debug>(resource: Res<R>) {
    info!("{:?}", resource);
}


#[derive(Component, Reflect, Debug, Default, PartialEq)]
pub struct JumpForce(pub Acceleration);
impl JumpForce {
    fn apply(mut query: Query<(&mut Acceleration, &JumpForce)>) {
        for (mut acceleration, force) in query.iter_mut() { 
            *acceleration += force.0;
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Frames(pub u32);

#[derive(Resource, Reflect, Debug, Default, PartialEq, Eq)]
pub struct JumpedLast(Stopwatch, u32);
impl JumpedLast {
    fn tick(time: Res<Time>, mut last_jumped: ResMut<JumpedLast>) {
        last_jumped.0.tick(time.delta());
        last_jumped.1 += 1;
    }

    pub fn register_jump(mut jumped_last: ResMut<JumpedLast>) { jumped_last.0.reset(); jumped_last.1 = 0; }
    pub fn elapsed(&self) -> Duration { self.0.elapsed() } 
    pub fn elapsed_frames(&self) -> Frames { Frames(self.1) } 
    // pub fn elapsed_secs(&self) -> f32 { self.0.elapsed_secs() }
    // pub fn elapsed_secs_f64(&self) -> f64 { self.0.elapsed_secs_f64() }
}


fn check_jump(
    mut player_state_next: ResMut<NextState<PlayerState>>,
    player_state: Res<State<PlayerState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    const JUMP_BUTTONS: [KeyCode; 3] = [KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp];
    match player_state.get() {
        PlayerState::Jumping => if !input.any_pressed(JUMP_BUTTONS) { player_state_next.set(PlayerState::Falling); },
        PlayerState::Falling => return,
        PlayerState::Idle |
        PlayerState::Walking => {
            if input.any_pressed(JUMP_BUTTONS) { 
                player_state_next.set(PlayerState::Jumping); 
            }
        }
    }
}

fn align_grounded_check_with_player_state(
    mut player_state_next: ResMut<NextState<PlayerState>>,
    player_state: Res<State<PlayerState>>,
    check: Query<&KinematicCharacterControllerOutput>,
    last_jumped: Res<JumpedLast>,
) {
    if let Ok(check) = check.get_single() {
        // info!("{}, {:?}", check.grounded, last_jumped.elapsed_frames());
    }
    match player_state.get() {
        PlayerState::Idle => return,
        PlayerState::Walking => return,
        PlayerState::Falling |
        PlayerState::Jumping => {
            // could move to a different system, but that would mean the state changes a frame later!
            // in the first few frames, it still thinks it is grounded!
            let check = check.get_single().unwrap();
            // info!(check.grounded);
            if check.grounded && last_jumped.elapsed_frames() > Frames(5) { 
                player_state_next.set(PlayerState::Idle); 
            }
        }
    }
}