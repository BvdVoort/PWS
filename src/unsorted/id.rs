use bevy::{prelude::{Component, Entity}, reflect::Reflect};
use std::marker::PhantomData;

#[derive(Component, Reflect)]
#[derive(Clone, Copy)]
pub struct Id<T>(Entity, #[reflect(ignore)] PhantomData<T>);
impl<T> From<Entity> for Id<T>
{
    fn from(value: Entity) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Id<T>
{
    pub fn new(entity: Entity) -> Self { Self(entity, PhantomData) }
    pub fn entity(&self) -> Entity { self.0 }
}

// should this be implemented or not?
// impl<T> Default for Id<T>
// {
//     fn default() -> Self {
//         Self(Entity::PLACEHOLDER, PhantomData)
//     }
// }

impl<T> From<Id<T>> for Entity
{
    fn from(value: Id<T>) -> Self {
        value.id()
    }
}

impl<T> From<&Id<T>> for Entity
{
    fn from(value: &Id<T>) -> Self {
        value.id()
    }
}