use bevy::{app::{Plugin, PreStartup}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{Bundle, Entity, NextState, ResMut, Trigger, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, CollisionGroups, Group};

use crate::{collision::LocalGroupNames, enemies::{ColliderBundle, ObservableColliderBundle}, game_flow::GameState, unsorted::{Promise, PromiseProcedure}};


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

struct Finish;
impl PromiseProcedure for Finish {
    fn resolve_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        world
            .commands()
            .entity(entity)
            .insert(
                ObservableColliderBundle::from(ColliderBundle {
                    collider: Collider::cuboid(5.0, 10.0),
                    collision_groups: CollisionGroups {
                        memberships: Group::GROUP_32,
                        filters: Group::PLAYER,
                    }
                }),
            )
            .observe(|_trigger: Trigger<CollisionEvent>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Completed);
            });
    }
}