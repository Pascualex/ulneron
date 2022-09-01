use bevy::prelude::*;

use crate::{
    protocol::{events::{ControllerEvent, ControllerEventData}},
    server::resources::PlayerInfo,
    server::resources::{GameState, PlayersInfo},
};

pub fn controller(
    mut reader: EventReader<ControllerEvent>,
    state: Res<GameState>,
    mut players_info: ResMut<PlayersInfo>,
) {
    for event in reader.iter() {
        match &event.data {
            ControllerEventData::Join(uuid) => {
                if matches!(*state, GameState::Lobby) && event.id == players_info.vec.len() {
                    players_info.vec.push(PlayerInfo::new(*uuid));
                }
            }
            ControllerEventData::Action(action) => {
                if event.id < players_info.vec.len() {
                    players_info.vec[event.id].action = action.clone();
                }
            }
        }
    }
}
