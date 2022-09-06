use bevy::prelude::*;

use crate::client::controller::{
    events::{ControllerEventData, ControllerUpstreamEvent},
    resources::ControllerInfo,
};

pub fn setup(
    controller_info: Res<ControllerInfo>,
    mut writer: EventWriter<ControllerUpstreamEvent>,
) {
    let data = ControllerEventData::Info(controller_info.uuid);
    writer.send(ControllerUpstreamEvent::new_local(data));
}
