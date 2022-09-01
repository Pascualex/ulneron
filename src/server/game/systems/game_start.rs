use bevy::prelude::*;

use crate::server::game::resources::GameState;

pub fn game_start(input: Res<Input<KeyCode>>, mut state: ResMut<GameState>) {
    if matches!(*state, GameState::Lobby) && input.pressed(KeyCode::Space) {
        *state = GameState::Ready;
    }
}
