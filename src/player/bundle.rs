use bevy::prelude::Bundle;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::prelude::GravityScale;

use crate::unsorted::Promise;

use super::{physics::{Acceleration, Velocity}, promise_procedure::Player};


#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    
    player: Promise<Player>,
    gravity_scale: GravityScale,
    acceleration: Acceleration,
    velocity: Velocity,
}