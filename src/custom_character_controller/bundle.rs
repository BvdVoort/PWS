use bevy::prelude::Bundle;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::prelude::GravityScale;

use crate::unsorted::Promise;
use super::character_promise::Character;


#[derive(Default, Bundle, LdtkEntity)]
pub struct CharacterBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    
    player: Promise<Character>,
    gravity_scale: GravityScale,
}