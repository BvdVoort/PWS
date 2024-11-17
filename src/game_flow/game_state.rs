use bevy::prelude::States;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    // LoadingScreen,
    // MainMenu,
    // LoadingLevel // or can this be the loading screen?
    Playing,
    Completed,
    Defeated,
}