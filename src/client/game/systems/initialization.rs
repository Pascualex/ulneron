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
    let offset = 0.3 * players_info.uuids.len().saturating_sub(1) as f32;
    for i in 0..players_info.uuids.len() {
        let entity = commands
            .spawn((
                Position::from_xy(-offset + 0.6 * i as f32, 0.0),
                Velocity::new(),
                Size::new(0.25),
                Stats::new(2.0),
                Agent::new(),
                Weapons::from_hertz(10, 150.0, 3.0),
                Player::new(i),
            ))
            .id();
        player_entities.vec.push(entity);
    }
    spawner.multiplier = players_info.uuids.len() as u32;
}
