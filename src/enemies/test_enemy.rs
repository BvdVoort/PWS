use std::{fmt::Debug, marker::PhantomData, time::Duration};

use bevy::{app::{App, Plugin, PostStartup, PreStartup, Startup, Update}, ecs::{component::{self, ComponentId, Tick}, observer, query, system::{IntoObserverSystem, ObserverSystem}, world::{self, DeferredWorld}}, log::error, math::{IVec2, Vec2}, prelude::{in_state, BuildChildren, Bundle, Commands, Component, DespawnRecursiveExt, Entity, Event, FromWorld, IntoSystem, IntoSystemConfigs, Local, Mut, NextState, Observer, Query, Res, ResMut, Resource, SpatialBundle, Transform, Trigger, With, World}, reflect::GetField, scene::ron::value, sprite::{Sprite, SpriteBundle}, text::{Text, Text2dBundle, TextStyle}, time::{Stopwatch, Time}, ui::Style, utils::{default, info}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, prelude::LdtkFields, utils::grid_coords_to_translation, EntityInstance, GridCoords, LdtkEntity, };
use bevy_rapier2d::{na::Translation, prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sensor}};

use crate::{character::{CharacterColision, ImageHandles}, collision::LocalGroupNames, game_flow::GameState, unsorted::{BevyPromiseResolver, Promise, PromiseProcedure}};
use super::{ColliderBundle, ObservableColliderBundle};

#[derive(Default, Bundle, LdtkEntity)]
struct TestEnemyBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    test_enemy: Promise<TestEnemy>,
    test_enemy_tag: TestEnemy,

    #[with(retrieve_patrol)]
    patrol: Patrol,
}

#[derive(Default, Component)]
struct TestEnemy;
impl TestEnemy {
    const CORNER_RADIUS: f32 = 4.0;
    const CAPSULE_HEIGHT: f32 = 8.0;

    const HALF_CAPSULE_HEIGHT: f32 = Self::CAPSULE_HEIGHT / 2.0;
}

#[derive(Component)]
struct PatrolWithoutTransform;

#[derive(Component)]
struct TransformWithoutPatrol;


pub struct TestEnemyPlugin;
impl Plugin for TestEnemyPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<TestEnemyBundle>("Mob")    
            .register_promise::<TestEnemy>()
            .add_systems(Update, handle_completion.run_if(in_state(GameState::Playing)).run_if(all_enemies_dead))            
            .add_systems(Update, move_patrol)            
            .add_systems(Startup, |world: &mut World| {
                world.register_component_hooks::<Patrol>().on_add(added_patrol);
                world.register_component_hooks::<Transform>().on_add(added_transform);
                // world.register_component_hooks::<GridCoords>().on_add(grid_coord_text);
                // world.register_component_hooks::<Patrol>().on_add(show_patrol_positions);
            })            
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
                },
            )
            .with_children(|children| {
                children.spawn(observer);
            })
            .remove_by_id(component_id);

        // let id = world.get::<EntityInstance>(entity).unwrap();
        // info(id.field_instances.clone());
    }
}

pub fn character_colision_handler(
    trigger: Trigger<CharacterColision>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    // damage query
) {
    let hit_details = trigger.event().hit.details.unwrap();
    let normal_from_self = hit_details.normal1;
    if normal_from_self.x.abs() == 1.0 || normal_from_self.y.is_sign_negative() {
        commands.entity(trigger.event().character).despawn_descendants();
        next_state.set(GameState::Defeated);
    }
    else {
        commands.entity(trigger.entity()).despawn_recursive();
    }    
}


#[derive(Component, Default)]
struct Patrol {
    targets: Vec<Vec2>,
    current_target: usize,
    speed: f32
}


impl From<Vec<Vec2>> for Patrol {
    fn from(targets: Vec<Vec2>) -> Self {
        Patrol {
            targets, 
            current_target: 0,
            speed: 16.0
        }
    }
}

impl Patrol {
    pub fn next(&mut self) {
        self.current_target += 1;
        self.current_target %= self.targets.len();
    }
}

const MAX_TILE_Y: i32 = 20;
fn retrieve_patrol(entity_instance: &EntityInstance) -> Patrol {
    
    if let Ok(patrol_slice) = entity_instance.get_maybe_points_field("patrol") {
        patrol_slice.iter().filter_map(|pt|{ 
            let IVec2 { x, y} =  (*pt)?;
            Some(grid_coords_to_translation(GridCoords::new(x, MAX_TILE_Y-y), TILESIZE))
        }).collect::<Vec<Vec2>>().into()
    }
    else { 
        Patrol::default()         
    }
}

fn spawn_coord_text(
    mut commands: Commands,
    query: Query<(Entity, &GridCoords)>,
) {
    if query.is_empty() { error!("No Coords Spawned") }
    for (entity, coord) in query.iter() {
        commands.entity(entity).insert(
            Text2dBundle {
                text: Text::from_section(format!("{}, {}", coord.x, coord.y), TextStyle::default()),
                transform: Transform::from_translation(grid_coords_to_translation(*coord, TILESIZE).extend(0.0)),
                ..default()
            }
        );
    }
}

fn grid_coord_text(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let coord = *world.get::<GridCoords>(entity).unwrap();
    world.commands().spawn(
        Text2dBundle {
            text: Text::from_section(format!("{}|{}", coord.x, coord.y), TextStyle{
                font_size: 6.0, ..Default::default()
            }),
            transform: Transform::from_translation(grid_coords_to_translation(coord, TILESIZE).extend(20.0)),
            ..default()
        }
    );
}

fn push_position(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let pos = world.get::<Transform>(entity).expect("A patrol should also have a transform component!").translation.truncate();
    info(format!("pos: {}", pos));
    world.get_mut::<Patrol>(entity).expect("This is a hook for on_add patrol!").targets.push(pos);
}

fn added_patrol(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let Some(Transform { translation, .. }) = world.get::<Transform>(entity) else { return; };
    let translation = translation.truncate();
    world.get_mut::<Patrol>(entity).expect("This is a hook for on_add patrol!").targets.push(translation);
}

fn added_transform(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let Transform { translation, .. } = world.get::<Transform>(entity).expect("This is a hook for on_add transform!");
    let translation = translation.truncate();
    let Some(mut patrol) = world.get_mut::<Patrol>(entity) else { return; };
    patrol.targets.push(translation);
}

    // fn show_patrol_positions(
//     mut world: DeferredWorld, entity: Entity, _component_id: ComponentId
// ) {
//     let patrol = world.get::<Patrol>(entity).expect("System should be a hook for this type").targets.clone();
//     let mut commands = world.commands();
//     for coord in patrol.iter() {
//         commands.spawn(
//             Text2dBundle {
//                 text: Text::from_section(format!("{}|{}", coord.x, coord.y), TextStyle{
//                     font_size: 6.0, ..Default::default()
//                 }),
//                 transform: Transform::from_translation(grid_coords_to_translation(*coord, TILESIZE).extend(20.0)),
//                 ..default()
//             }
//         );
//      }



    // for patrol in query.iter() {
    //     for coord in patrol.targets.iter() {
    //         commands.spawn(
    //             Text2dBundle {
    //                 text: Text::from_section(format!("{}|{}", coord.x, coord.y), TextStyle{
    //                     font_size: 6.0, ..Default::default()
    //                 }),
    //                 transform: Transform::from_translation(grid_coords_to_translation(*coord, TILESIZE).extend(20.0)),
    //                 ..default()
    //             }
    //         );
    //     }
    // }
// }



const TILESIZE: IVec2 = IVec2 { x: 16, y: 16 };
fn move_patrol(
    time: Res<Time>,
    mut query: Query<(&mut Patrol, &mut Transform)>,
    // mut stopwatch: Local<Stopwatch>
) {
    // stopwatch.tick(time.delta());
    // if stopwatch.elapsed() < Duration::from_secs(5) { return; }
    // stopwatch.reset();
    for (mut patrol, mut transfrom) in query.iter_mut() {
        let target = *patrol.targets.get(patrol.current_target).expect("No Patrol targets set?!");
        // transfrom.translation = target.extend(0.0);
        // patrol.next();
        // continue;

        let segment = target - transfrom.translation.truncate();
        let step = segment.normalize() * patrol.speed * time.delta_seconds();
        if step.length_squared() > segment.length_squared() {
            transfrom.translation = target.extend(transfrom.translation.z);
            patrol.next();
        }
        else {
            transfrom.translation += step.extend(0.0);
        }
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