use bevy::prelude::*;

use crate::server::lobby::resources::LobbyState;

pub fn lock(input: Res<Input<KeyCode>>, mut state: ResMut<LobbyState>) {
    if input.pressed(KeyCode::Space) {
        *state = LobbyState::Locking;
    }
}
