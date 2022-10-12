use bevy::prelude::*;

use crate::client::{
    game::{
        components::{Agent, Player, Position, Size, Stats, Velocity, Weapons},
        resources::{GameState, PlayerEntities, Spawner},
    },
    lobby::resources::{LobbyState, PlayersInfo},
};

pub fn initialization(
    lobby_state: Res<LobbyState>,
    players_info: Res<PlayersInfo>,
    mut game_state: ResMut<GameState>,
    mut player_entities: ResMut<PlayerEntities>,
    mut spawner: ResMut<Spawner>,
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
            .insert(Velocity::new())
            .insert(Size::new(0.25))
            .insert(Stats::new(5.0))
            .insert(Agent::new())
            .insert(Weapons::from_hertz(5, 250.0, 3.0))
            .insert(Player::new(id))
            .id();
        player_entities.vec.push(entity);
    }
    spawner.multiplier = players_info.uuids.len() as u32;
}
