use std::os::windows::process;

use bevy::{app::{Plugin, PreStartup}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{Bundle, Entity, NextState, ResMut, Trigger, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, CollisionGroups, Group, Sensor};

use crate::{collision::LocalGroupNames, game_flow::GameState, unsorted::Promise};

struct Finish;

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
                    .on_add(process_finish_promise);
            });
    }
}

fn process_finish_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    world
        .commands()
        .entity(entity)
        .insert((
            crate::enemies::ObservableCollider {
                collider: Collider::cuboid(5.0, 10.0),
                collision_groups: CollisionGroups {
                    memberships: Group::GROUP_32,
                    filters: Group::PLAYER,
                },
                active_physics_events: ActiveEvents::all(),
                collides_with: ActiveCollisionTypes::all(),
            }, Sensor
        )).observe(|_trigger: Trigger<CollisionEvent>, mut next_state: ResMut<NextState<GameState>>| {
            next_state.set(GameState::Completed);
        });
}