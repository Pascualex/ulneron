use bevy::prelude::*;

use crate::client::{
    lobby::resources::{LobbyState, PlayersInfo},
    networking::resources::Connection,
    ui::components::LobbyText,
};

pub fn lobby_menu(
    mut query: Query<&mut Text, With<LobbyText>>,
    state: Res<LobbyState>,
    connection: Option<Res<Connection>>,
    players_info: Res<PlayersInfo>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = match *state {
        LobbyState::Unlocked => match connection {
            Some(_) => match players_info.uuids.len() {
                0 => "Connecting".to_string(),
                1 => "Connected\n1 player".to_string(),
                p => format!("Connected\n{} players", p),
            },
            None => match players_info.uuids.len() {
                0 => "Starting".to_string(),
                1 => "Hosting\n1 player".to_string(),
                p => format!("Hosting\n{} players", p),
            },
        },
        LobbyState::Locked => "".to_string(),
    };
}
