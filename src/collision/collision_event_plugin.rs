use bevy::{app::{App, Plugin}, log::info, prelude::Trigger};
use bevy_rapier2d::prelude::CollisionEvent;

pub struct CollisionHanlerPlugin;
impl Plugin for CollisionHanlerPlugin {
    fn build(&self, app: &mut App) {
        app.observe(collision_handler);
    }
}

fn collision_handler(trigger: Trigger<CollisionEvent>) {
    info!("collided entity: {}", trigger.entity());
}