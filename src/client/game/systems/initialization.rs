use bevy::prelude::*;

use crate::client::{
    game::{
        components::{Player, Position, Velocity},
        resources::{GameState, PlayerEntities},
    },
    lobby::resources::{LobbyState, PlayersInfo},
};

pub fn initialization(
    lobby_state: Res<LobbyState>,
    players_info: Res<PlayersInfo>,
    mut game_state: ResMut<GameState>,
    mut player_entities: ResMut<PlayerEntities>,
    mut commands: Commands,
) {
    if !matches!(*game_state, GameState::Waiting) {
        return;
    }
    if !matches!(*lobby_state, LobbyState::Locked) {
        return;
    }
    *game_state = GameState::Running;
    for id in 0..players_info.uuids.len() {
        let entity = commands
            .spawn()
            .insert(Position::from_xy(0.0, 0.0))
            .insert(Velocity::from_xy(0.0, 0.0))
            .insert(Player::new(id))
            .id();
        player_entities.vec.push(entity);
    }
}
