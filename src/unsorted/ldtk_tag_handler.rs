use bevy::{
    app::{Plugin, PreStartup}, ecs::{
        component::ComponentId, world::DeferredWorld
    }, prelude::{Entity, World}
};
use bevy_ecs_ldtk::TileEnumTags;
use bevy_rapier2d::prelude::Collider;


mod enums {
    #![allow(unused)]
    pub trait EnumId { const ID: i32; }
    macro_rules! definitions {
        ($(enum $name:ident: $id:literal { $($variant_name:ident = $variant_value:literal)* })*) => {$(
            pub struct $name;
            impl EnumId for $name { const ID: i32 = $id; }
            impl $name {
                $(pub const $variant_name: &'static str = $variant_value;)*
            }
        )*};

    }
    definitions! {
        enum Collider: 117 {
            SOLID = "Solid"
        }
    }
}
use enums::EnumId;

pub struct LDTKEnumTagPluginCustom;
impl Plugin for LDTKEnumTagPluginCustom {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, |world: &mut World| {
            world
                .register_component_hooks::<TileEnumTags>()
                .on_add(proces_tile_enumtags);
        });
    }
}

fn proces_tile_enumtags(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let enumtags = world.get::<TileEnumTags>(entity).unwrap();
    let tags = enumtags.tags.clone(); // needs to be cloned so a world can be used to make commands. 
    let enumid = enumtags.source_enum_uid.expect("expected enum id on ldtk TileEnumTags component!");
    let mut commands = world.commands(); // needs to be a separate variable so it lives long enough. 
    let mut entity_commands = commands.entity(entity);
    match enumid {
        enums::Collider::ID => {
            for tag in tags.iter() { match tag as &str {
                enums::Collider::SOLID => { entity_commands.insert(Collider::cuboid(8., 8.)); }, // .8 should be a constant or something. Maybe based on the tilesize of the entity tile 
                _ => panic!("Unknown tag attached to {}! EnumId: {:?}; Tags: {:?}", entity, enumid, tags)
            }};
        }
        _ => panic!("Unknown enum attached to {}! EnumId: {:?}; Tags: {:?}", entity, enumid, tags)
    };
    entity_commands.remove::<TileEnumTags>();
}