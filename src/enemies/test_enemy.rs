use bevy::{app::{Plugin, PreStartup, Update}, ecs::{component::ComponentId, world::DeferredWorld}, prelude::{in_state, BuildChildren, Bundle, Component, Entity, IntoSystemConfigs, Local, NextState, Query, ResMut, SpatialBundle, With, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sensor};

use crate::{collision::LocalGroupNames, game_flow::GameState, unsorted::Promise};
use super::entity_bundles::ObservableCollider;

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
            .add_systems(PreStartup, |world: &mut World| {
                world
                    .register_component_hooks::<Promise<TestEnemy>>()
                    .on_add(process_test_enemy_promise);
            })
            .add_systems(Update, handle_completion.run_if(in_state(GameState::Playing)).run_if(all_enemies_dead))
            ;
    }
}

// #? MAYBE: impl a new Promise trait for TestEnemy {} // to make it more secure??
// #! TODO: Make a centrale obsever entity for all test enemies. 
fn process_test_enemy_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    world.commands().entity(entity)
        .insert((
            Collider::capsule_y(TestEnemy::HALF_CAPSULE_HEIGHT, TestEnemy::CORNER_RADIUS),
            CollisionGroups {
                memberships: Group::TEST_ENEMY,
                filters: Group::ALL & !Group::TEST_ENEMY_SENSOR
            },
        ))
        .with_children(|childeren| {
            let mut spatial_bundle = SpatialBundle::default();
            spatial_bundle.transform.translation.y = TestEnemy::HALF_CAPSULE_HEIGHT * 1.1;
            childeren.spawn((
                spatial_bundle,
                Sensor, 
                ObservableCollider {
                    collider: Collider::ball(TestEnemy::CORNER_RADIUS), 
                    collision_groups: CollisionGroups {
                        memberships: Group::TEST_ENEMY_SENSOR,
                        filters: Group::ALL & !Group::TEST_ENEMY & !Group::TEST_ENEMY_SENSOR,
                    },                     
                    active_physics_events: ActiveEvents::COLLISION_EVENTS,
                    collides_with: ActiveCollisionTypes::all(),
                }
            )).observe(test_enemy_handlers::sensor_player_collision_handler);
            
            childeren.spawn((
                SpatialBundle::default(),
                Sensor,
                ObservableCollider {
                    collider: Collider::capsule_y(TestEnemy::HALF_CAPSULE_HEIGHT, TestEnemy::CORNER_RADIUS * 1.5),
                    collision_groups: CollisionGroups {
                        memberships: Group::TEST_ENEMY_SENSOR,
                        filters: Group::ALL & !Group::TEST_ENEMY & !Group::TEST_ENEMY_SENSOR
                    },                
                    active_physics_events: ActiveEvents::COLLISION_EVENTS,
                    collides_with: ActiveCollisionTypes::all(),
            })).observe(test_enemy_handlers::self_player_colision);
        
        })
        // .observe(test_enemy_handlers::self_player_colision)
        .remove::<Promise<TestEnemy>>();
}


mod test_enemy_handlers {
    use bevy::{log::info, prelude::{Commands, DespawnRecursiveExt, Parent, Query, Res, Trigger}};
    use crate::player::{Player, PlayerCollision};

    pub fn sensor_player_collision_handler(
        trigger: Trigger<PlayerCollision>,
        mut commands: Commands,
        parent_ref_query: Query<&Parent>,
    ) {
        let parent = parent_ref_query
            .get(trigger.entity())
            .expect("The sensor of TestEnemy that collided should be a child of the actual TestEnemy entity!")
            .get();
        commands.entity(parent).despawn_recursive();
    }

    // doesn't get called!?
    pub fn self_player_colision(
        _trigger: Trigger<PlayerCollision>,
        mut commands: Commands,
        player: Res<Player>,
        // damage query
    ) {
        info!("Player died!");
        commands.entity(player.entity()).despawn_recursive();
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


// fn test_enemy_killing(
//     mut commands: Commands,
//     rapier_context: Res<RapierContext>,
//     collider_query: Query<Entity, With<TestEnemy>>,
//     player: Res<Player>,
// ) {
//     let player = player.entity();
//     for collider in collider_query.iter() {
//         for contact_pair in rapier_context.contact_pairs_with(collider) {
//             let other = {
//                 let first = contact_pair.collider1();
//                 if first == collider { contact_pair.collider2() }
//                 else { first }
//             };        
//             if other != player { continue; } 
//             commands.entity(collider).despawn();
//         }
//     }
// }