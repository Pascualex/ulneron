use bevy::prelude::*;

use crate::{
    protocol::events::{ControllerEvent, ControllerEventData},
    server::controller::resources::{PlayerInfo, PlayersInfo},
};

pub fn controller(mut reader: EventReader<ControllerEvent>, mut players_info: ResMut<PlayersInfo>) {
    for event in reader.iter() {
        match &event.data {
            ControllerEventData::Join(uuid) => {
                players_info.vec.push(PlayerInfo::new(*uuid));
            }
            ControllerEventData::Action(action) => {
                if event.id < players_info.vec.len() {
                    players_info.vec[event.id].action = action.clone();
                }
            }
        }
    }
}
