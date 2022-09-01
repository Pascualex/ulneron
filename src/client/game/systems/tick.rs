use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Player, Position, Velocity},
        resources::{GameState, PlayersInfo, Ticks},
    },
    protocol::events::GameEvent,
};

pub fn tick(
    mut reader: EventReader<GameEvent>,
    mut state: ResMut<GameState>,
    mut players_info: ResMut<PlayersInfo>,
    mut ticks: ResMut<Ticks>,
    mut commands: Commands,
) {
    if !ticks.vec.is_empty() {
        ticks.vec.remove(0);
    }

    for event in reader.iter() {
        match &event {
            GameEvent::Startup(startup) => {
                if state.started {
                    panic!("Received startup event while in game");
                }
                state.started = true;
                for (id, uuid) in startup.uuids.iter().enumerate() {
                    players_info.uuids.push(*uuid);
                    let entity = commands
                        .spawn()
                        .insert(Position::from_xy(0.0, 0.0))
                        .insert(Velocity::from_xy(0.0, 0.0))
                        .insert(Player::new(id))
                        .id();
                    players_info.entities.push(entity);
                }
            }
            GameEvent::Tick(tick) => {
                if !state.started {
                    panic!("Received tick event while not in game");
                }
                ticks.vec.push(tick.clone());
            }
        };
    }
}
