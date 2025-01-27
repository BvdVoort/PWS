use std::{ops::Mul, time::Duration};

use bevy::{
    app::{
        Plugin, 
        PreStartup, 
        PreUpdate, 
        Update
    }, asset::{
        AssetServer, 
        Handle
    }, color::Color, log::info, math::{Vec2, VectorSpace}, prelude::{
        Bundle, Commands, Component, Entity, Event, GamepadButtonType, Image, KeyCode, Local, Query, Res, Resource
    }, reflect::Reflect, sprite::{
        Sprite, 
        SpriteBundle
    }, time::{
        Stopwatch, 
        Time
    }, utils::default
};
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::{
    plugin::RapierConfiguration, 
    prelude::{
        ActiveCollisionTypes, 
        ActiveEvents, 
        Collider, 
        GravityScale, 
        KinematicCharacterController, 
        KinematicCharacterControllerOutput, 
        ShapeCastHit, 
        Velocity
    }
};
use leafwing_input_manager::{
    plugin::InputManagerPlugin, 
    InputManagerBundle,
    Actionlike, 
    prelude::{
        ActionState, 
        GamepadControlAxis, 
        InputMap, 
        KeyboardVirtualAxis, 
        WithAxisProcessingPipelineExt
    },
};

use crate::unsorted::{Promise, PromiseProcedure, BevyPromiseResolver};

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_ldtk_entity_with_promise::<Player>("Player")
            .add_plugins(InputManagerPlugin::<CharacterAction>::default())
            .add_systems(PreStartup, load_sprites)
            .add_systems(Update, (player_movement, character_colision))
            ;
    }
}



#[derive(Actionlike, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CharacterAction {
    #[actionlike(Axis)]   Move,
    #[actionlike(Button)] Jump,
}


#[derive(LdtkEntity, Bundle, Default)]
struct Player {
    promise: Promise<Self>
    // #[sprite_sheet_bundle]
    // sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

fn standard_character_input_map() -> InputManagerBundle<CharacterAction> {
    InputManagerBundle::with_map(
        InputMap::default()
            // .with_axis(InputAction::Move, KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS) // if you use both arrows and ad you can get double the axis value
            .with_axis(CharacterAction::Move, KeyboardVirtualAxis::AD)
            .with_axis(CharacterAction::Move, GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.1))

            .with(CharacterAction::Jump, KeyCode::Space)
            .with(CharacterAction::Jump, GamepadButtonType::South)
    )
}

impl PromiseProcedure for Player {
    fn resolve_promise<'w>(mut world: bevy::ecs::world::DeferredWorld<'w>, entity: bevy::prelude::Entity, component_id: bevy::ecs::component::ComponentId) {
        let player_texture = world.resource::<ImageHandles>().player.clone_weak();
        world
            .commands()
            .entity(entity)
            .insert((
                standard_character_input_map(),
                KinematicCharacterController::default(),
                Collider::capsule_y(4., 4.),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                Velocity::default(),
                GravityScale::default(),
                SpriteBundle {
                    texture: player_texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(8.0, 16.5)),
                        ..default()
                    },

                    ..default()
                },
            ));
        world.commands().entity(entity).remove_by_id(component_id);
    }
}

const MOVEMENT_VELOCITY: f32 = 200.0;
const JUMP_VELOCITY: f32 = 100.0;
const GRAVITY_INFLUENCE_JUMP: f32 = 0.3;
const COYOTE_TIME: Duration = Duration::from_millis(100);

fn player_movement(
    time: Res<Time>,
    physics: Res<RapierConfiguration>,
    mut jumped: Local<bool>,
    mut since_last_grounded: Local<Stopwatch>,
    mut query: Query<(
        &ActionState<CharacterAction>, 
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>, 
        &mut Velocity, 
        &GravityScale
    )>,
) {

    for 
    (
        input, 
        mut character_controller, 
        character_output, 
        mut velocity, 
        gravity_scale
    ) 
    in query.iter_mut() 
    {
        let mut gravity_scale = gravity_scale.0;
        let mut grounded = character_output
            .map(|output| output.grounded)
            .unwrap_or(false);

        if grounded { 
            since_last_grounded.reset(); 
        }
        else {
            since_last_grounded.tick(time.delta());
            grounded = since_last_grounded.elapsed() < COYOTE_TIME;  
        }

        if input.pressed(&CharacterAction::Jump) {
            gravity_scale *= GRAVITY_INFLUENCE_JUMP;
            if grounded && !(*jumped) { 
                velocity.linvel.y = JUMP_VELOCITY; 
                *jumped = true;
            }
        }
        
        let gravity = physics.gravity * gravity_scale;
        let acceleration = gravity;
        velocity.linvel += acceleration * time.delta_seconds();
        
        let input_axis_horizontal = input
            .axis_data(&CharacterAction::Move)
            .map(|axis| axis.value)
            .unwrap_or_default();

        velocity.linvel.x = input_axis_horizontal * MOVEMENT_VELOCITY;        
               
        if grounded {
            if !(*jumped) {
                velocity.linvel.y = 0.0;
            }
        }
        else {
            *jumped = false;
        }
        
        character_controller.translation = Some(velocity.linvel * time.delta_seconds());
    }
}

#[derive(Component, Reflect, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
enum Jump {
    #[default]
    Impossible,
    Possible,
    Started,
    Performing,
}

impl Jump {
    pub fn can_jump(&self) -> bool { *self == Self::Possible }
    pub fn try_jump(&mut self, jump_action: impl FnOnce()) {
        if self.can_jump() { unsafe { self.jump(jump_action) }; }
    }

    pub unsafe fn jump(&mut self, jump_action: impl FnOnce()) {
        jump_action();
        *self = Self::Started;
    }
}

#[derive(Component, Reflect, Debug, Default, PartialEq, Clone)]
struct JumpTracker {
    time_since_last: Stopwatch,
    state: Jump,
}

impl JumpTracker {
    const COYOTE_TIME: Duration = Duration::from_millis(100);

    fn can_jump(&self) -> bool { self.state.can_jump() || (self.state == Jump::Impossible && self.time_since_last.elapsed() < Self::COYOTE_TIME) } // COYOTE_TIME should be greater than the time since grounded!!
    fn try_jump(&mut self, jump_action: impl FnOnce()) { 
        if self.can_jump() { unsafe { self.jump(jump_action); } } 
    }

    pub unsafe fn jump(&mut self, jump_action: impl FnOnce()) {
        unsafe { self.state.jump(jump_action); }
        self.time_since_last.reset();
    }
}

#[derive(Event, Debug)]
pub struct CharacterColision{
    pub hit: ShapeCastHit,
    pub character: Entity
}

fn character_colision(
    mut commands: Commands,
    mut query: Query<(Entity, &KinematicCharacterControllerOutput)>
) {
    for (entity, controller_output) in query.iter_mut() {
        for colision in controller_output.collisions.clone() {
            commands.trigger_targets(CharacterColision{ hit: colision.hit, character: entity }, colision.entity);
        }
    }
}

#[derive(Resource)]
pub(crate) struct ImageHandles {
    pub player: Handle<Image>,
    pub test_enemy: Handle<Image>,
}

fn load_sprites(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let player = server.load("Character.png");
    let test_enemy = server.load("Enemy.png");
    commands.insert_resource(ImageHandles { player, test_enemy });
}