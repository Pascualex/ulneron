use bevy::prelude::*;

#[derive(Resource)]
pub enum LobbyState {
    Unlocked,
    Locked,
}
