use bevy::prelude::*;

use crate::{
    protocol::{events::{GameEvent, LobbyEvent}, data::{Lobby, Startup, Tick}},
    server::resources::{GameState, PlayersInfo},
};

pub fn tick(
    mut state: ResMut<GameState>,
    players_info: Res<PlayersInfo>,
    mut lobby_writer: EventWriter<LobbyEvent>,
    mut game_writer: EventWriter<GameEvent>,
) {
    match *state {
        GameState::Lobby => {
            let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
            let lobby = Lobby::new(uuids);
            lobby_writer.send(LobbyEvent::new(lobby));
        }
        GameState::Ready => {
            *state = GameState::Game;
            let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
            let startup = Startup::new(uuids);
            game_writer.send(GameEvent::Startup(startup));
        }
        GameState::Game => {
            let actions = players_info.vec.iter().map(|i| i.action.clone()).collect();
            let tick = Tick::new(actions);
            game_writer.send(GameEvent::Tick(tick));
        }
    };
}
