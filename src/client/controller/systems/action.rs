use bevy::prelude::*;

use crate::{
    client::controller::resources::ActionBuilder,
    protocol::events::{ControllerEvent, ControllerEventData},
};

pub fn action_writer(builder: Res<ActionBuilder>, mut writer: EventWriter<ControllerEvent>) {
    let action = builder.action.clone();
    let data = ControllerEventData::Action(action);
    writer.send(ControllerEvent::new_local(data));
}
