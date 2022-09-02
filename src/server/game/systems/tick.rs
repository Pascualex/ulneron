use bevy::prelude::*;

use crate::{
    protocol::{data::Tick, events::GameDownstreamEvent},
    server::{controller::resources::PlayersInfo, lobby::resources::LobbyState},
};

pub fn tick(
    state: ResMut<LobbyState>,
    players_info: Res<PlayersInfo>,
    mut game_writer: EventWriter<GameDownstreamEvent>,
) {
    if !matches!(*state, LobbyState::Locked) {
        return;
    }
    let actions = players_info.vec.iter().map(|i| i.action.clone()).collect();
    let tick = Tick::new(actions);
    let event = GameDownstreamEvent::new(tick);
    game_writer.send(event);
}
