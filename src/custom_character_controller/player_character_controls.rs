use std::time::Duration;

use bevy::{app::{Plugin, PreUpdate, Update}, input::ButtonInput, log::info, math::Vec2, prelude::{Component, KeyCode, Query, Res, ResMut, Resource, With}, reflect::Reflect, time::Time};
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::{GravityScale, KinematicCharacterController, KinematicCharacterControllerOutput}};

use super::{character_promise::PlayerTag, input::MovementAxis, physics::{Acceleration, Velocity}};

pub struct PlayerControlsPlugin;
impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<CharacterControllerConfig>()
            .register_type::<JumpTracker>()
            .register_type::<JumpConfig>()
            .register_type::<MovementConfig>()
            .add_systems(PreUpdate, sync_grounded)
            .add_systems(Update, (
                player_gravity,
                player_movement,
                player_jump,
            ))
            ;
    }
}

#[derive(Resource, Reflect)]
pub struct MovementConfig {
    pub grounded_speed: f32,
    pub air_speed: f32,
}

#[derive(Resource, Reflect, Default)]
pub struct JumpTracker(Jumping);
impl JumpTracker {
    pub fn is_grounded(&self) -> bool { self.0.is_grounded() }
    pub fn is_jumping(&self) -> bool { self.0.is_jumping() }
    pub fn sync_grounded(&mut self, grounded: bool) { self.0.update(grounded); }
}

pub fn sync_grounded(mut query: Query<&KinematicCharacterControllerOutput>, mut jump_tracker: ResMut<JumpTracker>) {
    for output in query.iter_mut() {
        jump_tracker.sync_grounded(output.grounded);
    }
}

pub fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    mut jump_tracker: ResMut<JumpTracker>,
    jump_config: Res<JumpConfig>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<&mut Velocity, With<PlayerTag>>
) {
    if !jump_tracker.is_grounded() || !input.any_pressed([KeyCode::Space]) { return; }
    let jump_velocity = jump_config.height / jump_config.duration.as_secs_f32() - rapier_config.gravity.y * jump_config.duration.as_secs_f32(); // #! TODO: gravity scale and gravity not in y direction  
    jump_tracker.0 = Jumping::Starting;
    for mut velocity in query.iter_mut() {
        *velocity += Velocity::up(jump_velocity);
        info!("{:?}", velocity);
    }
}


pub fn player_movement(
    input_axis: Res<MovementAxis>,
    movement_config: Res<MovementConfig>,
    mut query: Query<&mut Acceleration, With<PlayerTag>>
) {
    for mut acceleration in query.iter_mut() {
        *acceleration += Acceleration::horizontal(input_axis.horizontal() * movement_config.grounded_speed)
    }
}

pub fn player_gravity(
    time: Res<Time>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(&mut Velocity, &GravityScale), With<KinematicCharacterController>>
) {
    for (mut velocity, gravity_scale) in query.iter_mut() {
        *velocity += Acceleration::from_vec2(rapier_config.gravity) * gravity_scale.0 * time.delta();
        info!("Gravity");
    }
}




// pub fn player_input(
//     time: Res<Time>,
//     input: Res<ButtonInput<KeyCode>>,
//     config: Res<CharacterControllerConfig>,
//     jump_config: Res<JumpConfig>,
//     rapier_config: Res<RapierConfiguration>,
//     mut query: Query<(
//         &mut KinematicCharacterController,
//         // &KinematicCharacterControllerOutput,
//         &mut Velocity,
//         &mut GravityScale,
//         &mut CustomCharacterData,
//         // &mut JumpCount,
//         // &CoyoteStopwatch,
//         // &CollisionInfo,
//     )>,
// ) { 
//     // info!("{}", query.is_empty());
//     for (
//         mut controller,
//         // controller_output,
//         mut velocity,
//         gravity_scale,
//         mut data,
//         // mut vel_x_smoothing,
//         // mut jump_count,
//         // coyote_stopwatch,
//         // collisions,
//     ) in query.iter_mut()
//     {
//         let mut jump_velocity = 2.0 * jump_config.height /  jump_config.duration.as_secs_f32(); // s = 0.5 × v_gem × t => v_gem = (2 * s) / t;
//         let mut jump_acceleration = 2.0 * jump_config.height / jump_config.duration.as_secs_f32().powi(2); // s = 0.5 × a × t^2 => a = (2 * s) / t^2; should it use acceleration!?
        
//         let grounded = data.jump_state.is_grounded();
//         // Reset Y velocity if touching (!above!) or below
        
//         if grounded { 
//             velocity= 0.0; }
//             if !input.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) { 
//                 jump_velocity = 0.0; 
//             }
//         else {  jump_velocity = 0.0; }

        
//         let mut input_raw: f32 = 0.0;
//         if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { input_raw -= 1.0; } 
//         if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { input_raw += 1.0; }

//         // if grounded && input.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space])
//         // // || coyote_stopwatch.0.elapsed() <= controller.coyote_time (of type duration??)
//         // {
//         //     jump_acceleration = 0.0;
//         //     data.jump_state = Jumping::Starting;
//         //     velocity.linvel += jump_velocity;
//         // }

//         // Smooth x movement
//         // let target_velocity_x = input_raw.x * controller.move_speed * time.delta_seconds();
//         // let acceleration_time = if collisions.below {
//             //     controller.acceleration_time_grounded
//             // } else {
//                 //     controller.acceleration_time_airborne
//         // };
//         // velocity.0.x = smooth_damp(
//             //     velocity.0.x,
//             //     target_velocity_x,
//             //     &mut vel_x_smoothing.0,
//             //     acceleration_time,
//             //     f32::INFINITY,
//             //     time.delta_seconds(),
//             // );
//         let jump_velocity = -rapier_config.gravity.normalize() * jump_velocity; // jump in other direction as gravity with calculated speed
//         let jump_acceleration = -rapier_config.gravity.normalize() * jump_acceleration; // jump in other direction as gravity with calculated speed
//         let jump_acceleration = jump_velocity * time.delta_seconds(); // had elepsed time not delta time lol 

//         let jump_acceleration = jump_acceleration * input_raw;
//         let jump_velocity = jump_velocity * input_raw;
//         let gravity_acceleration = rapier_config.gravity * gravity_scale.0;
//         let movement_acceleration = Vec2::new(
//             input_raw * config.movement.speed_ground,
//             0.0
//         );
//         velocity.linvel += jump_velocity + (gravity_acceleration + /*jump_acceleration +*/ movement_acceleration) * time.delta_seconds(); // m/s^2 * s = m/s

//         controller.translation = Some(controller.translation.unwrap_or_default() + velocity.linvel * time.delta_seconds()); // m/s * s = m
//     }
// }

// pub fn update_custom_character_data(mut query: Query<(&mut CustomCharacterData, &KinematicCharacterControllerOutput)>) {
//     for (mut data, output) in query.iter_mut() {
//         data.desired_translation = output.desired_translation;
//         data.jump_state.update(output.grounded);
//         // for collision in output.collisions.iter() {
            
//         // }
//     }
// }

#[derive(Component, Reflect, Debug, Default)]
pub struct CustomCharacterData {
    pub jump_state: Jumping,
    pub desired_translation: Vec2,
}

#[derive(Debug, Default, PartialEq, Eq, Reflect)]
pub enum Jumping {
    #[default]
    Impossible,
    Possible,
    Starting,
    Performing,
}

impl Jumping {
    pub fn update(&mut self, grounded: bool) {
        match self {
            Self::Possible => if !grounded { *self = Self::Impossible; },
            Self::Starting => if !grounded { *self = Self::Performing; },
            Self::Impossible |
            Self::Performing => if grounded { *self = Self::Possible },
        }
    }

    pub fn is_grounded(&self) -> bool { *self == Self::Possible }
    pub fn is_jumping(&self) -> bool {
        *self == Self::Performing || *self == Self::Starting
    }
}

#[derive(Resource, Reflect)]
pub struct CharacterControllerConfig {
    pub movement: Movement,
    pub jump_gravity_scale: f32,
    pub fall_gravity_scale: f32,
}

#[derive(Resource, Reflect, Debug)]
pub struct JumpConfig {
    pub height: f32,
    pub duration: Duration,
}

impl JumpConfig {
    pub const fn new(height: f32, duration: Duration) -> Self {
        Self { height, duration }
    }
}

#[derive(Reflect, Debug)]
pub struct Movement {
    pub speed_ground: f32,
    pub speed_air: f32
}