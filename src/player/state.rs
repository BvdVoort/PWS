use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    // Running,
    Jumping,
    Falling,
}