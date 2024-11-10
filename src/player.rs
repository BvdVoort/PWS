use bevy::{
    app::{Plugin, Update}, input::{ButtonInput, InputPlugin}, math::Vec2, prelude::{
        default, Added, Bundle, Commands, Entity, IntoSystemConfigs, KeyCode, Query, Res 
    }
};
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle
};
use bevy_rapier2d::prelude::{
    CharacterLength, Collider, KinematicCharacterController, Velocity 
};
use super::{unsorted::Promise, input::input_sets::StandardInput};


#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player: Promise<Player>,

    velocity: Velocity,
}

#[derive(Default)]
struct Player;


fn proces_player_promise(
    mut commands: Commands,
    players: Query<Entity, Added<Promise<Player>>>
) {
    // There should only be one player. So we could use player.single()
    for entity in players.iter()
    {
        commands.entity(entity).insert((
            Collider::capsule_y(4., 4.),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.08),
                up: Vec2::Y,
                ..default()
            }
        ));
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<InputPlugin>()
        {
            app.add_plugins(InputPlugin);
        }
        
        app
            .register_ldtk_entity::<PlayerBundle>("Player")     
            .add_systems(Update, proces_player_promise)
            .add_systems(Update, player_movement.in_set(StandardInput))
            ;
    }
}


const GRAVITY: f32 = 0.1;
const MOVEMENT_SCALER: f32 = 10.;
fn player_movement(
    mut controllers: Query<&mut KinematicCharacterController>,
    input: Res<ButtonInput<KeyCode>>,    
) {
    for mut controller in controllers.iter_mut() 
    {
        let mut movement = Vec2::ZERO;
        if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            movement += Vec2::NEG_X;
        }
        if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            movement += Vec2::X;
        }
        if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            movement += Vec2::Y;
        }
        if input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            movement += Vec2::NEG_Y;
        }

        if input.pressed(KeyCode::Space) {
            movement += Vec2::Y * MOVEMENT_SCALER
        }

        movement.y -= GRAVITY;
        controller.translation = Some(
            controller.translation.unwrap_or_default() + movement.normalize_or_zero() * MOVEMENT_SCALER
        );
    }
}