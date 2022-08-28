use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Player, Position, Velocity},
        resources::{GameState, PlayerInfo, PlayersInfo, Ticks},
    },
    protocol::{data::DownstreamData, events::DownstreamEvent},
};

pub fn downstream_reader(
    mut reader: EventReader<DownstreamEvent>,
    mut state: ResMut<GameState>,
    mut players_info: ResMut<PlayersInfo>,
    mut ticks: ResMut<Ticks>,
    mut commands: Commands,
) {
    if !ticks.vec.is_empty() {
        ticks.vec.remove(0);
    }

    for event in reader.iter() {
        match &event.data {
            DownstreamData::Startup(startup) => {
                if !matches!(*state, GameState::Lobby) {
                    panic!("Startup event while not in lobby");
                }
                *state = GameState::Game;
                players_info.vec.clear();
                for (id, uuid) in startup.iter().enumerate() {
                    let entity = commands
                        .spawn()
                        .insert(Position::from_xy(0.0, 0.0))
                        .insert(Velocity::from_xy(0.0, 0.0))
                        .insert(Player::new(id))
                        .id();
                    let player_info = PlayerInfo::new(*uuid, entity);
                    players_info.vec.push(player_info);
                }
            }
            DownstreamData::Tick(tick) => {
                if !matches!(*state, GameState::Game) {
                    panic!("Tick event while not in game");
                }
                ticks.vec.push(tick.clone());
            }
        };
    }
}
