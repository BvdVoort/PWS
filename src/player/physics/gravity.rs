use bevy::prelude::{Query, Res};
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::KinematicCharacterController};

use super::option_add_assign::OptionAddAssignExtension;


pub fn apply_gravity_to_character(rapier_config: Res<RapierConfiguration>, mut controller_query: Query<&mut KinematicCharacterController>)
{
    for mut controller in controller_query.iter_mut() {
        controller.translation.add_assign(rapier_config.gravity);
    }
}