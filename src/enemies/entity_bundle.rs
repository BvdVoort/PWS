use bevy::prelude::Bundle;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups
};

#[derive(Bundle)]
pub struct ObservableCollider {
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub active_physics_events: ActiveEvents,
    pub collides_with: ActiveCollisionTypes,
}