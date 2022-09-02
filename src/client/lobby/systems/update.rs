use bevy::prelude::*;

use crate::{client::lobby::resources::PlayersInfo, protocol::events::LobbyDownstreamEvent};

pub fn update(
    mut reader: EventReader<LobbyDownstreamEvent>,
    mut players_info: ResMut<PlayersInfo>,
) {
    for event in reader.iter() {
        players_info.uuids = event.lobby.uuids.clone();
    }
}
