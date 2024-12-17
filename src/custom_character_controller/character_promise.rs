use bevy::{math::Vec2, prelude::Component, utils::default};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, GravityScale, KinematicCharacterController};

use crate::unsorted::{Uid, PromiseProcedure};

use super::physics::{Acceleration, Velocity};

#[derive(Component, Clone, Copy)]
pub struct PlayerTag;

pub struct Character;
impl PromiseProcedure for Character {
    fn resolve_promise<'w>(mut world: bevy::ecs::world::DeferredWorld<'w>, entity: bevy::prelude::Entity, component_id: bevy::ecs::component::ComponentId) {
        let mut commands = world.commands();
        commands.insert_resource(Uid::<Character>::from(entity));
        commands
            .entity(entity)
            .insert((
                Collider::capsule_y(4., 4.),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                // RigidBody::KinematicPositionBased, // velocity based??
                KinematicCharacterController {
                    // offset: CharacterLength::Absolute(0.08),
                    up: Vec2::Y,
                    ..default()
                }, PlayerTag,
                Velocity::default(),
                Acceleration::default(),
                GravityScale::default(),
            ))
            .remove_by_id(component_id);
    }
}