use std::{fmt::Debug, marker::PhantomData};

use bevy::{app::{App, Plugin, PreStartup, Startup, Update}, ecs::{component::{ComponentId, Tick}, observer, system::{IntoObserverSystem, ObserverSystem}, world::DeferredWorld}, prelude::{in_state, BuildChildren, Bundle, Commands, Component, DespawnRecursiveExt, Entity, Event, FromWorld, IntoSystemConfigs, Local, Mut, NextState, Observer, Query, Res, ResMut, Resource, SpatialBundle, Trigger, With}, scene::ron::value, utils::info};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sensor};

use crate::{character::CollidedWithCharacter, collision::LocalGroupNames, game_flow::GameState, unsorted::{BevyPromiseResolver, Promise, PromiseProcedure}};
use super::{ColliderBundle, ObservableColliderBundle};

#[derive(Default, Bundle, LdtkEntity)]
struct TestEnemyBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    test_enemy: Promise<TestEnemy>,
    test_enemy_tag: TestEnemy,
}

#[derive(Default, Component)]
struct TestEnemy;
impl TestEnemy {
    const CORNER_RADIUS: f32 = 4.0;
    const CAPSULE_HEIGHT: f32 = 8.0;

    const HALF_CAPSULE_HEIGHT: f32 = Self::CAPSULE_HEIGHT / 2.0;
}

pub struct TestEnemyPlugin;
impl Plugin for TestEnemyPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<TestEnemyBundle>("Mob")    
            .register_promise::<TestEnemy>()
            .add_systems(Update, handle_completion.run_if(in_state(GameState::Playing)).run_if(all_enemies_dead))            
            ;
    }
}

// #! TODO: Make a centrale obsever entity for all test enemies. 
impl PromiseProcedure for TestEnemy {
    fn resolve_promise(mut world: DeferredWorld, entity: Entity, component_id: ComponentId) {
        let mut observer = Observer::new(character_colision_handler);
        observer.watch_entity(entity);
        world
            .commands()
            .entity(entity)
            .insert(
                ColliderBundle {
                collider: Collider::capsule_y(TestEnemy::HALF_CAPSULE_HEIGHT, TestEnemy::CORNER_RADIUS),
                collision_groups: CollisionGroups {
                    memberships: Group::TEST_ENEMY,
                    filters: Group::ALL & !Group::TEST_ENEMY_SENSOR
                },
            })
            .with_children(|children| {
                children.spawn(observer);
            })
            .remove_by_id(component_id);
    }
}

pub fn character_colision_handler(
    trigger: Trigger<CollidedWithCharacter>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    // damage query
) {
    if trigger.event().hit.details.unwrap().normal1.x.abs() == 1.0 {
        commands.entity(trigger.event().character).despawn_descendants();
        next_state.set(GameState::Defeated);
    }
    else {
        commands.entity(trigger.entity()).despawn_recursive();
    }    
}


fn all_enemies_dead(enemies: Query<(), With<TestEnemy>>, mut enemies_spawned_param: Local<bool>) -> bool
{
    let enemies_spawned = enemies_spawned_param.clone();
    if enemies_spawned { enemies.is_empty() } else {
        *enemies_spawned_param = !enemies.is_empty();
        return false;
    }
}

fn handle_completion(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Completed);
}