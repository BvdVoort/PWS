use bevy::{app::{Plugin, PreStartup}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{BuildChildren, Bundle, Entity, NextState, ResMut, SpatialBundle, Transform, Trigger, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, CollisionGroups, Group};

use crate::{character::CharacterColision, collision::LocalGroupNames, enemies::{ColliderBundle, ObservableColliderBundle}, game_flow::GameState, unsorted::{Promise, PromiseProcedure}};


#[derive(Default, Bundle, LdtkEntity)]
struct FinishBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    test_enemy: Promise<Finish>,
}

pub struct FinishPlugin;
impl Plugin for FinishPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<FinishBundle>("Finish")
            .add_systems(PreStartup, |world: &mut World| { 
                world
                    .register_component_hooks::<Promise<Finish>>()
                    .on_add(Finish::resolve_promise);
                }
            );
    }
}

const TILESIZE: f32 = 8.0;

struct Finish;
impl PromiseProcedure for Finish {
    fn resolve_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        world
            .commands()
            .entity(entity)
            .with_children(|children| {
                children.spawn((
                    SpatialBundle::from_transform(Transform::from_xyz(0.0, TILESIZE*1.5, 0.0)),
                    Collider::cuboid(TILESIZE / 2.0, TILESIZE * 3.0),
                    CollisionGroups {
                        memberships: Group::GROUP_32,
                        filters: Group::PLAYER,
                    }
                ))
                .observe(|_trigger: Trigger<CharacterColision>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Completed);
                });
            });
    }
}