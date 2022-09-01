use bevy::prelude::*;

use crate::client::{
    game::resources::{GameState, PlayersInfo},
    networking::resources::Connection,
};

pub fn lobby(
    game_state: Res<GameState>,
    connection: Option<Res<Connection>>,
    players_info: Res<PlayersInfo>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = match game_state.started {
        false => match connection {
            Some(_) => match players_info.uuids.len() {
                1 => "Connected\n1 player".to_string(),
                p => format!("Connected\n{} players", p),
            },
            None => match players_info.uuids.len() {
                1 => "Hosting\n1 player".to_string(),
                p => format!("Hosting\n{} players", p),
            },
        },
        true => "".to_string(),
    };
}
