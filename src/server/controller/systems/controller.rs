use bevy::prelude::*;

use crate::{
    client::controller::events::{ControllerEventData, ControllerUpstreamEvent},
    server::controller::resources::{ControllerInfo, ControllersInfo},
};

pub fn controller(
    mut reader: EventReader<ControllerUpstreamEvent>,
    mut controllers_info: ResMut<ControllersInfo>,
) {
    for event in reader.iter() {
        match &event.data {
            ControllerEventData::Info(uuid) => {
                controllers_info.vec.push(ControllerInfo::new(*uuid));
            }
            ControllerEventData::Action(action) => {
                if event.id < controllers_info.vec.len() {
                    controllers_info.vec[event.id].action = action.clone();
                }
            }
        }
    }
}
