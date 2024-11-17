use bevy::{app::{Plugin, PreStartup, Update}, ecs::{component::ComponentId, query, world::DeferredWorld}, log::info, prelude::{in_state, resource_exists, BuildChildren, Bundle, Commands, Component, DespawnRecursiveExt, Entity, EventReader, IntoSystemConfigs, Local, NextState, Parent, Query, Res, ResMut, SpatialBundle, With, World}};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::{prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, Sensor}, rapier::prelude::CollisionEventFlags};

use crate::{enemies, game_flow::GameState, player::Player, unsorted::Promise};


#[derive(Default, Bundle, LdtkEntity)]
struct TestEnemyBundle {
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
    test_enemy: Promise<TestEnemy>,
    test_enemy_tag: TestEnemy,
}

#[derive(Default, Component)]
struct TestEnemy;

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
            .add_systems(Update, test_enemy_killing.run_if(in_state(GameState::Playing)).run_if(resource_exists::<Player>))
            .add_systems(Update, handle_completion.run_if(all_enemies_dead))
            ;
    }
}

// impl a new Promise trait for TestEnemy {} // to make it more secure??
fn process_test_enemy_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    world.commands().entity(entity)
        .insert((Collider::capsule_y(4., 3.), TestEnemy))
        .with_children(|childeren| {
            let mut spatial_bundle = SpatialBundle::default();
            spatial_bundle.transform.translation.y = 6.;
            childeren.spawn((
                Collider::ball(3.), Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                spatial_bundle,
            ));
        })
        .remove::<Promise<TestEnemy>>();
}

// should also handle killing of player somewhere!
pub fn test_enemy_killing(
    mut collision_events: EventReader<CollisionEvent>,
    player: Res<Player>,
    mut commands: Commands,
    child_query: Query<&Parent>,
) {
    let player_entity = player.entity();
    // let player_entity = Entity::PLACEHOLDER;
    for collision in collision_events.read() {
        // info!("{collision:?}");
        let CollisionEvent::Started(entity_1, entity_2, CollisionEventFlags::SENSOR) = collision else { continue };
        let other = {
            if player_entity == *entity_1 { *entity_2 }
            else if player_entity == *entity_2 { *entity_1 }
            else { continue; }
        };
        if let Ok(parent) = child_query.get(other) {
            commands.entity(parent.get()).despawn_recursive();
        }
        else {
            commands.entity(other).despawn_recursive();
        }
        // info!("{collision:?}. With: {other}");
        
    }
}

fn all_enemies_dead(enemies: Query<(), With<TestEnemy>>, mut enemies_spawned_param: Local<bool>) -> bool
{
    let enemies_spawned = enemies_spawned_param.clone();
    if enemies_spawned { enemies.is_empty() }
    else {
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