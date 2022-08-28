use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Player, Position, Velocity},
        resources::{PlayerInfo, PlayersInfo, Ticks},
    },
    protocol::{data::DownstreamData, events::DownstreamEvent},
};

pub fn downstream_reader(
    mut reader: EventReader<DownstreamEvent>,
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
                ticks.vec.push(tick.clone());
            }
        };
    }
}
