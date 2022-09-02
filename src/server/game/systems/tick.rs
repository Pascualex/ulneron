use bevy::prelude::*;

use crate::{
    protocol::{
        data::{Startup, Tick},
        events::GameDownstreamEvent,
    },
    server::{controller::resources::PlayersInfo, game::resources::GameState},
};

pub fn tick(
    mut state: ResMut<GameState>,
    players_info: Res<PlayersInfo>,
    mut game_writer: EventWriter<GameDownstreamEvent>,
) {
    let event = match *state {
        GameState::Ready => {
            *state = GameState::Game;
            let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
            let startup = Startup::new(uuids);
            GameDownstreamEvent::Startup(startup)
        }
        GameState::Game => {
            let actions = players_info.vec.iter().map(|i| i.action.clone()).collect();
            let tick = Tick::new(actions);
            GameDownstreamEvent::Tick(tick)
        }
        _ => return,
    };
    game_writer.send(event);
}
