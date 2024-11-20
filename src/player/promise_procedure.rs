use bevy::{math::Vec2, utils::default};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, KinematicCharacterController};

use crate::unsorted::{Uid, PromiseProcedure};

pub struct Player;
impl PromiseProcedure for Player {
    fn invoke<'w>(mut world: bevy::ecs::world::DeferredWorld<'w>, entity: bevy::prelude::Entity, component_id: bevy::ecs::component::ComponentId) {
        let mut commands = world.commands();
        commands.insert_resource(Uid::<Player>::from(entity));
        commands
            .entity(entity)
            .insert((
                Collider::capsule_y(4., 4.),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                KinematicCharacterController {
                    // offset: CharacterLength::Absolute(0.08),
                    up: Vec2::Y,
                    ..default()
                }
            ))
            .remove_by_id(component_id);
    }
}