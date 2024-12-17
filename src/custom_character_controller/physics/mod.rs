mod scalar;
mod distance;
mod velocity;
mod acceleration;

use std::fmt::Formatter;

pub use scalar::Scalar;
pub use distance::Distance;
pub use velocity::Velocity;
pub use acceleration::Acceleration;

use bevy_rapier2d::{plugin::PhysicsSet, prelude::{KinematicCharacterController, KinematicCharacterControllerOutput}};
use bevy::{app::{Plugin, PostUpdate}, log::info, prelude::{IntoSystemConfigs, Query, Res}, reflect::Reflect, time::Time};

fn apply_acceleration_to_velocity(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        *velocity = *acceleration * time.delta();
        // info!("{:?}", acceleration)
    }
}

fn apply_velocity_to_character_translation(time: Res<Time>, mut query: Query<(&mut KinematicCharacterController, &Velocity)>) {
    for (mut controller, velocity) in query.iter_mut() {
        controller.translation += *velocity * time.delta();
        info!("applied velocity: {:?}", controller.translation.unwrap_or_default());
    }
}

mod local_compare_temp {
    use std::cmp::Ordering;

    pub fn min<T: PartialOrd>(v1: T, v2: T) -> T {
        match v1.partial_cmp(&v2) {
            Some(Ordering::Less) => v1,
            Some(Ordering::Equal)|
            Some(Ordering::Greater) => v2,
            None => if v1.partial_cmp(&v1).is_some() { v1 } else { v2 },
        }
    }

    pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
        match v1.partial_cmp(&v2) {
            Some(Ordering::Less) => v2,
            Some(Ordering::Equal)|
            Some(Ordering::Greater) => v1,
            None => if v1.partial_cmp(&v1).is_some() { v1 } else { v2 },
        }
    }
} pub use local_compare_temp::{min, max};

use super::player_character_controls::JumpTracker;

fn zero_velocity_when_blocked(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput)>
) {
    for (mut velocity, output) in query.iter_mut() {
        if output.effective_translation.y == 0.0 {
            velocity.meters_per_second.y = 0.0;
        }
        if output.effective_translation.x == 0.0 {
            velocity.meters_per_second.x = 0.0;
        }
    } 
}

fn zero_acceleration_when_blocked(
    mut query: Query<(&mut Acceleration, &KinematicCharacterControllerOutput)>
) {
    for (mut acceleration, output) in query.iter_mut() {
        if output.effective_translation.y == 0.0 {
            acceleration.meters_per_second_squared.y = 0.0;
        }
        if output.effective_translation.x == 0.0 {
            acceleration.meters_per_second_squared.x = 0.0;
        }
    } 
}

fn resistance_horizontal(
    mut query: Query<&mut Acceleration>
) {
    for mut acceleration in query.iter_mut() {
        let resistance = acceleration.clone() * 0.1;
        *acceleration -= resistance;
        // info!("{:?}", acceleration)
    }
}

pub struct CharacterPhysicsPlugin;
impl Plugin for CharacterPhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .add_systems(PostUpdate, (
                apply_acceleration_to_velocity,
                apply_velocity_to_character_translation,
            ).chain().in_set(PhysicsSet::StepSimulation))
            // .add_systems(PostUpdate, (
            //     zero_velocity_when_blocked,
            //     zero_acceleration_when_blocked,
            //     resistance_horizontal,
            // ).in_set(PhysicsSet::SyncBackend))

            .add_plugins(bevy_inspector_egui::quick::ResourceInspectorPlugin::<JumpTracker>::new())
        ;
    }
}