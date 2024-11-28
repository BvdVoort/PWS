use std::marker::PhantomData;

use bevy::{app::{App, Plugin, PreStartup, Startup, Update}, ecs::{component::{ComponentId, Tick}, observer, system::{IntoObserverSystem, ObserverSystem}, world::DeferredWorld}, prelude::{in_state, BuildChildren, Bundle, Commands, Component, Entity, Event, FromWorld, IntoSystemConfigs, Local, Mut, NextState, Observer, Query, Res, ResMut, Resource, SpatialBundle, Trigger, With}, scene::ron::value, utils::info};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group};

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

#[derive(Resource)]
struct ObserverResource<E, T>
where 
    E: 'static + Event,
    T: Bundle,
{
    observer: Entity,
    event: PhantomData<E>,
    target: PhantomData<T>,
}

impl<E, T> FromWorld for ObserverResource<E, T>
where 
    E: 'static + Event,
    T: Bundle,
{
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        ObserverResource { 
            observer: world.commands().spawn_empty().id(),
            event: PhantomData,
            target: PhantomData,
        }
    }
}

impl<E, T> ObserverResource<E, T> 
where 
    E: 'static + Event,
    T: Bundle,
{
    pub fn init_handler(&mut self, mut commands: Commands, handler: impl IntoObserverSystem<E, T, ()>) {
        commands.entity(self.observer).insert(Observer::new(handler));
    }

    pub fn entity(&self) -> Entity {
        self.observer
    }
}

pub struct TestEnemyPlugin;
impl Plugin for TestEnemyPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity::<TestEnemyBundle>("Mob")    
            .register_promise::<TestEnemy>()
            .add_systems(Update, handle_completion.run_if(in_state(GameState::Playing)).run_if(all_enemies_dead))            
            // .init_resource::<ObserverResource<CollidedWithCharacter, TestEnemy>>()
            //.add_systems(
            //     Startup, 
            //     |mut commands: Commands, mut observer: Res<ObserverResource<CollidedWithCharacter, TestEnemy>>| {
            //         observer.init_handler(commands, |trigger: Trigger<CollidedWithCharacter, TestEnemy>| {
            //             trigger.
            //         });
            //     }
            // )
            ;
    }
}

// #! TODO: Make a centrale obsever entity for all test enemies. 
impl PromiseProcedure for TestEnemy {
    fn resolve_promise(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        // let observer = world
        //     .get_resource::<ObserverResource<CollidedWithCharacter, TestEnemy>>()
        //     .unwrap()
        //     .entity()
        //     ;

        // let mut observer_component;
        // let mut added;
        // let mut last_changed;
        
        
        // let mut observer_entity = world.entity_mut(observer);
        // let observer = if let Some(observer) 
        // = observer_entity.get_mut::<Observer<CollidedWithCharacter, TestEnemy>>() 
        // { 
        //     observer
        // } 
        // else
        // {
        //     added = Tick::default();
        //     last_changed = Tick::default();
        //     observer_component = Observer::new(
        //         |trigger: Trigger<CollidedWithCharacter, TestEnemy>| {
        //             info(trigger.entity());
        //         }
        //     );
        //     Mut::<Observer<CollidedWithCharacter, TestEnemy>>::new(
        //         &mut observer_component,
        //         &mut added,
        //         &mut last_changed,
        //         Tick::default(),
        //         Tick::default(),
        //     )
        // };

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
            .with_children(|childeren| {
                let mut spatial_bundle = SpatialBundle::default();
                spatial_bundle.transform.translation.y = TestEnemy::HALF_CAPSULE_HEIGHT * 1.3;

                childeren.spawn((
                    spatial_bundle,
                    ObservableColliderBundle::from(ColliderBundle {
                        collider: Collider::ball(TestEnemy::CORNER_RADIUS), 
                        collision_groups: CollisionGroups {
                            memberships: Group::TEST_ENEMY_SENSOR,
                            filters: Group::ALL & !Group::TEST_ENEMY & !Group::TEST_ENEMY_SENSOR,
                        },                     
                    })
                ))
                .observe(test_enemy_handlers::sensor_player_collision_handler);
                
                childeren.spawn((
                    SpatialBundle::default(),
                    ObservableColliderBundle::from(ColliderBundle {
                        collider: Collider::capsule_y(TestEnemy::HALF_CAPSULE_HEIGHT, TestEnemy::CORNER_RADIUS * 1.1),
                        collision_groups: CollisionGroups {
                            memberships: Group::TEST_ENEMY_SENSOR,
                            filters: Group::ALL & !Group::TEST_ENEMY & !Group::TEST_ENEMY_SENSOR
                        },                
                    })
                ))
                .observe(test_enemy_handlers::self_player_colision);
            
            })
            .remove::<Promise<TestEnemy>>();
    }
}

mod test_enemy_handlers {
    use bevy::{prelude::{Commands, DespawnRecursiveExt, NextState, Parent, Query, Res, ResMut, Trigger}, utils::info};
    use crate::{character::CollidedWithCharacter, game_flow::GameState, player::{Player, PlayerCollision}, unsorted::Uid};

    pub fn sensor_player_collision_handler(
        trigger: Trigger<CollidedWithCharacter>,
        mut commands: Commands,
        parent_ref_query: Query<&Parent>,
    ) {
        info(trigger.event().entity());
        let parent = parent_ref_query
            .get(trigger.entity())
            .expect("The sensor of TestEnemy that collided should be a child of the actual TestEnemy entity!")
            .get();
        commands.entity(parent).despawn_recursive();
    }

    // doesn't get called when collider isn't a sensor!?
    pub fn self_player_colision(
        _trigger: Trigger<CollidedWithCharacter>,
        mut commands: Commands,
        player: Res<Uid<Player>>,
        mut next_state: ResMut<NextState<GameState>>,
        // damage query
    ) {
        info(_trigger.event().entity());
        commands.entity(player.entity()).despawn_recursive();
        next_state.set(GameState::Defeated);
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