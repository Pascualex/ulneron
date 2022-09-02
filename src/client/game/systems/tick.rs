use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Player, Position, Velocity},
        resources::{GameState, PlayerEntities, Ticks},
    },
    protocol::events::GameDownstreamEvent,
};

pub fn tick(
    mut reader: EventReader<GameDownstreamEvent>,
    mut state: ResMut<GameState>,
    mut players_entities: ResMut<PlayerEntities>,
    mut ticks: ResMut<Ticks>,
    mut commands: Commands,
) {
    if !ticks.vec.is_empty() {
        ticks.vec.remove(0);
    }

    for event in reader.iter() {
        match &event {
            GameDownstreamEvent::Startup(startup) => {
                if state.started {
                    panic!("Received startup event while in game");
                }
                state.started = true;
                for id in 0..startup.uuids.len() {
                    let entity = commands
                        .spawn()
                        .insert(Position::from_xy(0.0, 0.0))
                        .insert(Velocity::from_xy(0.0, 0.0))
                        .insert(Player::new(id))
                        .id();
                    players_entities.vec.push(entity);
                }
            }
            GameDownstreamEvent::Tick(tick) => {
                if !state.started {
                    panic!("Received tick event while not in game");
                }
                ticks.vec.push(tick.clone());
            }
        };
    }
}
