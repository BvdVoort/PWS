use bevy::prelude::{Commands, Event, EventReader, Res};
use bevy_rapier2d::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags};

use super::promise_procedure::Player;
use crate::unsorted::Uid;


#[derive(Event)]
pub struct PlayerCollision;

pub fn player_collision_handler(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    player: Res<Uid<Player>>,
) {
    let player_entity = player.entity();
    for collision in collision_events.read() {
        let CollisionEvent::Started(entity_1, entity_2, CollisionEventFlags::SENSOR) = collision else { continue };
        commands.trigger_targets(PlayerCollision, {
            if player_entity == *entity_1 { *entity_2 } else 
            if player_entity == *entity_2 { *entity_1 } else { continue; /*No player collision*/ }
        });
    }
}