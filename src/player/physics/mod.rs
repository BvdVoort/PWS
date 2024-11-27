mod option_add_assign;
mod scalar;
mod velocity;
mod acceleration;
mod gravity;
mod walk;
mod jump;
mod distance;

use acceleration::{apply_acceleration, reset_acceleration};
use gravity::apply_gravity_to_character;
use jump::JumpPlugin;
use velocity::apply_velocity;

use bevy::{app::{First, Plugin, PostUpdate, Update}, math::{Vec2, VectorSpace}, prelude::{IntoSystemConfigs, Query}};
use bevy_rapier2d::{parry::query, prelude::{KinematicCharacterControllerOutput, PhysicsSet::StepSimulation}};
use bevy_rapier2d::prelude::PhysicsSet;

use walk::apply_walk_movement;

#[allow(unused_imports)] pub use scalar::Scaler;
#[allow(unused_imports)] pub use velocity::Velocity;
#[allow(unused_imports)] pub use acceleration::Acceleration;
#[allow(unused_imports)] pub use jump::JumpForce;

pub struct PlayerPhysicsPlugin;
impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<Acceleration>()
            .register_type::<Velocity>()
            .add_plugins(JumpPlugin)
            .add_systems(PostUpdate, (
                (
                    apply_acceleration,
                    apply_velocity,
                )
                .chain()
                .in_set(StepSimulation), 
                (
                    reset_acceleration, // should be reset force but force isnt yet implemented!
                )
                .in_set(PhysicsSet::Writeback),    
            ))
            .add_systems(First, |mut query: Query<(&KinematicCharacterControllerOutput, &mut Velocity)>|{
                for (output, mut physics) in query.iter_mut() {
                    match output.desired_translation - output.effective_translation {
                        Vec2 { x: 0., y: 0. } => return,
                        Vec2 { x: 0., .. } => physics.linear.x = 0.,
                        Vec2 { y: 0., .. } => physics.linear.y = 0.,
                        _ => physics.linear = Vec2::ZERO,
                    }
                }
            })
            .add_systems(Update, (
                apply_gravity_to_character,
                apply_walk_movement,
            ))
            ;
    }
}