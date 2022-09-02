use bevy::prelude::*;

use crate::server::controller::{resources::*, systems::*};

pub struct ServerControllerPlugin;

impl Plugin for ServerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllersInfo>()
            .add_system_to_stage(CoreStage::Update, controller);
    }
}
