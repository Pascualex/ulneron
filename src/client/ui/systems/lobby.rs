use bevy::prelude::*;

use crate::client::{
    controller::resources::ControllerInfo,
    game::resources::{GameState, PlayersInfo},
};

pub fn lobby(
    game_state: Res<GameState>,
    controller_info: Res<ControllerInfo>,
    players_info: Res<PlayersInfo>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = match game_state.started {
        false => match controller_info.id {
            None => "Joining".to_string(),
            Some(_) => match players_info.uuids.len() {
                1 => "Connected\n1 player ready".to_string(),
                p => format!("Connected\n{} players ready", p),
            },
        },
        true => "".to_string(),
    };
}
