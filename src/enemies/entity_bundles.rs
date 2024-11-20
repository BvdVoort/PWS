use bevy::prelude::Bundle;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Sensor
};

#[derive(Bundle)]
pub struct ObservableColliderBundle {
    pub collider_bundle: ColliderBundle,
    pub active_physics_events: ActiveEvents,
    pub collides_with: ActiveCollisionTypes,
    pub sensor: Sensor,
}

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
}

impl From<ColliderBundle> for ObservableColliderBundle {
    fn from(collider_bundle: ColliderBundle) -> Self {
        Self {
            collider_bundle,
            active_physics_events: ActiveEvents::all(), 
            collides_with: ActiveCollisionTypes::all(), 
            sensor: Sensor,
        }
    }
}