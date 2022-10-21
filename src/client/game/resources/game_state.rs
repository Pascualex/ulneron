use bevy::prelude::*;

#[derive(Resource)]
pub enum GameState {
    Waiting,
    Running,
}
