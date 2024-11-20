use bevy::{app::{App, PreStartup}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{Component, Entity, World}};
use std::marker::PhantomData;

/// [Component] that promises further procesing of an [entity].
/// 
/// This [component] should thus be immediately removed when procesing is completed!
/// `Promise` is a wrapper around [phantomdata] which is <i>zero sized.</I>
/// 
/// [entity]: bevy::prelude::Entity
/// [component]: Component
/// [phantomdata]: PhantomData
/// 
#[derive(Component)]
pub struct Promise<T: PromiseProcedure>(PhantomData<T>);

impl<T: PromiseProcedure> Default for Promise<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub trait PromiseProcedure {
    fn invoke<'w>(world: DeferredWorld<'w>, entity: Entity, component_id: ComponentId);
}

// #! TODO: fix the promise resolver
// pub trait BevyPromiseResolver {
//     fn register_promise<T: PromiseProcedure + Send + Sync>(app: &mut self) -> &mut Self;
// }
    
// impl BevyPromiseResolver for App {
//     fn register_promise<T: PromiseProcedure + Send + Sync>(app: &mut self) {
//         app.add_systems(PreStartup, |world: &mut World| { 
//             world
//                 .register_component_hooks::<Promise<T>>()
//                 .on_add(T::invoke);
//             }
//         );
//     }
// }