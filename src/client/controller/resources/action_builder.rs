use bevy::prelude::*;

use crate::client::controller::data::Action;

#[derive(Default, Resource)]
pub struct ActionBuilder {
    pub action: Action,
}
