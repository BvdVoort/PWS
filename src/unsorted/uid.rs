use bevy::{prelude::{Entity, Resource}, reflect::Reflect};
use std::marker::PhantomData;


#[derive(Resource, Reflect, Clone, Copy)]
pub struct Uid<T>(Entity, #[reflect(ignore)] PhantomData<T>);
impl<T> From<Entity> for Uid<T>
{
    fn from(value: Entity) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Uid<T>
{
    pub fn new(entity: Entity) -> Self { Self(entity, PhantomData) }
    pub fn entity(&self) -> Entity { self.0 }
    pub unsafe fn default() -> Self {
        Self(Entity::PLACEHOLDER, Default::default())
    }
}

impl<T> From<Uid<T>> for Entity
{
    fn from(value: Uid<T>) -> Self {
        value.entity()
    }
}