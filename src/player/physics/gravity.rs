use bevy::prelude::{Query, Res};
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::GravityScale};
use super::Acceleration;


pub fn apply_gravity_to_character(rapier_config: Res<RapierConfiguration>, mut controller_query: Query<(&mut Acceleration, Option<&GravityScale>)>)
{
    for (mut acceleration, gravity_scale) in controller_query.iter_mut() {
        *acceleration += Acceleration{ linear: rapier_config.gravity * gravity_scale.map_or(GravityScale::default().0, |scale|scale.0) };
    }
}