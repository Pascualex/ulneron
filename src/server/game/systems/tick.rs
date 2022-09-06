use bevy::prelude::*;

use crate::{
    client::controller::data::Action,
    server::{
        controller::resources::ControllersInfo,
        game::{data::Tick, events::GameDownstreamEvent},
        lobby::resources::{LobbyState, PlayersInfo},
    },
};

pub fn tick(
    state: ResMut<LobbyState>,
    players_info: Res<PlayersInfo>,
    controllers_info: Res<ControllersInfo>,
    mut writer: EventWriter<GameDownstreamEvent>,
) {
    if !matches!(*state, LobbyState::Locked) {
        return;
    }
    let mut actions = Vec::new();
    for id in 0..players_info.uuids.len() {
        let action = match controllers_info.vec.get(id) {
            Some(ci) => ci.action.clone(),
            None => Action::new(),
        };
        actions.push(action);
    }
    let tick = Tick::new(actions);
    let event = GameDownstreamEvent::new(tick);
    writer.send(event);
}
