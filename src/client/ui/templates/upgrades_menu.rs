use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::components::UpgradesText;

pub fn upgrades_menu(mut commands: EntityCommands, asset_server: &AssetServer) {
    commands
        .insert_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
        )
        .insert(UpgradesText);
}
