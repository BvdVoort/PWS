use bevy::{
    app::{Plugin, PreStartup}, ecs::{
        component::ComponentId, world::DeferredWorld
    }, log::warn, prelude::{BuildChildren, Bundle, Entity, SpatialBundle, Transform, World}, transform::commands, utils::default
};
use bevy_ecs_ldtk::{TileEnumTags, TileMetadata};
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
        enum Collider: 107 {
            SOLID = "Solid"
            TOP_SLAB = "TopSlab"
        }
    }
}
use enums::EnumId;
use serde::{Deserialize, Serialize};

pub struct LDTKEnumTagPluginCustom;
impl Plugin for LDTKEnumTagPluginCustom {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, |world: &mut World| {
            world
                .register_component_hooks::<TileEnumTags>()
                .on_add(process_tile_enumtags);
            world
                .register_component_hooks::<TileMetadata>()
                .on_add(process_tile_metadata);
        });
    }
}

const TILESIZE: f32 = 16.0;
const HALF_TILESIZE: f32 = TILESIZE/2.0;
const QUARTER_TILESIZE: f32 = HALF_TILESIZE/2.0;
fn process_tile_enumtags(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let enumtags = world.get::<TileEnumTags>(entity).unwrap();
    let tags = enumtags.tags.clone(); // needs to be cloned so a world can be used to make commands. 
    let enumid = enumtags.source_enum_uid.expect("expected enum id on ldtk TileEnumTags component!");
    let mut commands = world.commands(); // needs to be a separate variable so it lives long enough. 
    let mut entity_commands = commands.entity(entity);
    match enumid {
        enums::Collider::ID => {
            for tag in tags.iter() { match tag as &str {
                enums::Collider::SOLID => { entity_commands.insert(Collider::cuboid(HALF_TILESIZE, HALF_TILESIZE)); },
                enums::Collider::TOP_SLAB => { entity_commands.with_children(|children| {
                    children.spawn((
                        SpatialBundle::from_transform(Transform::from_xyz(0.0, QUARTER_TILESIZE, 0.0)),
                        Collider::cuboid(HALF_TILESIZE, QUARTER_TILESIZE)
                    ));
                });},
                _ => warn!("Unknown tag attached to {}! EnumId: {:?}; Tags: {:?}", entity, enumid, tags)
            }};
        }
        _ => warn!("Unknown enum attached to {}! EnumId: {:?}; Tags: {:?}", entity, enumid, tags)
    };
    entity_commands.remove::<TileEnumTags>();
}

mod serde_defaults {
    pub fn one() -> f32 {1.0}
    pub fn half() -> f32 {0.5}
} pub use serde_defaults::*;

#[derive(Serialize, Deserialize)]
struct PointData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ShapeData {
    Square {
        #[serde(default = "one")] width: f32,
        #[serde(default = "one")] height: f32,
    },
    Circle { 
        #[serde(default = "half")] radius: f32,
    }
}

impl ShapeData {
    pub fn collider(&self) -> Collider {
        match self {
            ShapeData::Square { width, height } => Collider::cuboid(0.5*width, 0.5*height),
            ShapeData::Circle { radius } => Collider::ball(*radius),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
enum PivotData {
    Offset {
        #[serde(default)] x: f32,
        #[serde(default)] y: f32,
    },
    #[serde(untagged)] None,
}

#[derive(Serialize, Deserialize, Debug)]
struct ColliderData {
    shape: ShapeData,
    pivot: PivotData,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileMetadataParsed {
    collider: ColliderData
}

impl TileMetadataParsed {
    fn components(&self) -> impl Bundle {
        self.collider.shape.collider()
    } 
}


fn process_tile_metadata(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let mut metadata = world.get::<TileMetadata>(entity).unwrap().data.split_whitespace().collect::<String>();
    if !metadata.starts_with('{') { 
        metadata.insert(0, '{');    
        metadata.push('}'); 
    }

    let metadata = serde_json::from_str::<TileMetadataParsed>(&metadata).expect("Parsing tile metadata went wrong!");
    let mut commands = world.commands();
    let mut entity_commands = commands.entity(entity);

    entity_commands.insert(metadata.components());
    entity_commands.remove::<TileMetadata>();
}