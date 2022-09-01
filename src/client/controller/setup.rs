use bevy::prelude::*;

use crate::{
    client::controller::resources::ControllerInfo,
    protocol::events::{ControllerEvent, ControllerEventData},
};

pub fn setup(controller_info: Res<ControllerInfo>, mut writer: EventWriter<ControllerEvent>) {
    let data = ControllerEventData::Join(controller_info.uuid);
    writer.send(ControllerEvent::new_local(data));
}
