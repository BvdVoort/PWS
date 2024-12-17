use bevy::{math::Vec2, prelude::Component, utils::default};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, KinematicCharacterController};

use crate::unsorted::{Uid, PromiseProcedure};

use super::physics::{Acceleration, JumpForce};

#[derive(Component, Clone, Copy)]
pub struct PlayerTag;

pub struct Player;
impl PromiseProcedure for Player {
    fn resolve_promise<'w>(mut world: bevy::ecs::world::DeferredWorld<'w>, entity: bevy::prelude::Entity, component_id: bevy::ecs::component::ComponentId) {
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
                }, PlayerTag,
                JumpForce(Acceleration::UP * 2000.)
            ))
            .remove_by_id(component_id);
    }
}