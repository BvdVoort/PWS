use bevy::{app::{App, PreStartup}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{Bundle, Component, Entity, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, prelude::LdtkEntity};
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
#[derive(Component, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[component(storage = "SparseSet")]
pub struct Promise<T: PromiseProcedure>(PhantomData<T>);

impl<T: PromiseProcedure> Default for Promise<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub trait PromiseProcedure {
    fn resolve_promise<'w>(world: DeferredWorld<'w>, entity: Entity, component_id: ComponentId);
}
trait BevyOnly {}
#[allow(private_bounds)]
pub trait BevyPromiseResolver: BevyOnly {
    #[allow(dead_code)]
    fn register_promise<P: PromiseProcedure + Sync + Send + 'static>(&mut self) -> &mut Self;
    
    #[allow(dead_code)]
    fn register_ldtk_entity_with_promise<P: LdtkEntity + Bundle + PromiseProcedure + Sync + Send + 'static>(&mut self, entity_identifier: &str) -> &mut Self;
}

impl BevyOnly for App {}
impl BevyPromiseResolver for App {
    fn register_promise<P: PromiseProcedure + Sync + Send + 'static>(&mut self) -> &mut App {
        self.add_systems(PreStartup, |world: &mut World| { 
            world
                .register_component_hooks::<Promise<P>>()
                .on_add(P::resolve_promise);
            }
        );
        self
    }
    
    fn register_ldtk_entity_with_promise<P: LdtkEntity + Bundle + PromiseProcedure + Sync + Send + 'static>(&mut self, entity_identifier: &str) -> &mut Self {
        self
            .register_ldtk_entity::<P>(entity_identifier)
            .register_promise::<P>()
    }
}