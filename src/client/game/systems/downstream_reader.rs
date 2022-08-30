use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Player, Position, Velocity},
        resources::{GameState, PlayersInfo, Ticks},
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
            DownstreamData::Lobby(lobby) => {
                players_info.uuids = lobby.clone();
            }
            DownstreamData::Startup(startup) => {
                if state.started {
                    panic!("Received startup event while in game");
                }
                state.started = true;
                players_info.uuids = startup.clone();
                players_info.entities.clear();
                for (id, _) in startup.iter().enumerate() {
                    let entity = commands
                        .spawn()
                        .insert(Position::from_xy(0.0, 0.0))
                        .insert(Velocity::from_xy(0.0, 0.0))
                        .insert(Player::new(id))
                        .id();
                    players_info.entities.push(entity);
                }
            }
            DownstreamData::Tick(tick) => {
                if !state.started {
                    panic!("Received tick event while not in game");
                }
                ticks.vec.push(tick.clone());
            }
        };
    }
}
