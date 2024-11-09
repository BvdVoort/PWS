use bevy::prelude::Component;
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
#[derive(Default, Component)]
pub struct Promise<T>(PhantomData<T>);