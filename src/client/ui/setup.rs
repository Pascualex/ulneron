use bevy::prelude::*;

use crate::client::ui::templates::overlay;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    overlay(commands.spawn(), &asset_server);
}
