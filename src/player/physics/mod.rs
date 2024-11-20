mod option_add_assign;
mod velocity;
mod acceleration;
mod gravity;
mod walk;
mod jump;
mod scheldules;

use acceleration::{apply_acceleration, reset_acceleration};
use gravity::apply_gravity_to_character;
use jump::apply_jump_acceleration;
use velocity::apply_velocity;
// use scheldules::PlayerPhysicsCalculate;

use bevy::{app::{First, MainScheduleOrder, Plugin, PostUpdate}, prelude::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet}};


#[allow(unused_imports)] pub use velocity::Velocity;
#[allow(unused_imports)] pub use acceleration::Acceleration;
#[allow(unused_imports)] pub use scheldules::PlayerPhysicsConfigure;
use walk::apply_walk_movement;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct PlayerPhysicsForceSetter;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct PlayerPhysicsReset;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
struct PlayerPhysicsCalculate;


pub struct PlayerPhysicsPlugin;
impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<Acceleration>()
            .register_type::<Velocity>()
            .configure_sets(First, (
                PlayerPhysicsReset.before(PlayerPhysicsForceSetter),
                PlayerPhysicsForceSetter.before(PlayerPhysicsCalculate),
                PlayerPhysicsCalculate
            ))
            .add_systems(First, reset_acceleration.in_set(PlayerPhysicsReset))
            .add_systems(First, apply_velocity.after(apply_acceleration).in_set(PlayerPhysicsCalculate))
            .add_systems(First, apply_acceleration.before(apply_velocity).in_set(PlayerPhysicsCalculate))
            .add_systems(PlayerPhysicsConfigure, (
                apply_gravity_to_character,
                apply_walk_movement,
                apply_jump_acceleration
            ))
            ;
    }
}