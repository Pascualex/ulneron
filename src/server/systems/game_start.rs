use bevy::prelude::*;

use crate::server::resources::GameState;

pub fn game_start(input: Res<Input<KeyCode>>, mut state: ResMut<GameState>) {
    if input.pressed(KeyCode::Space) {
        state.started = true;
    }
}
