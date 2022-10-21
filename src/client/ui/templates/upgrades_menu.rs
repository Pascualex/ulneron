use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::components::UpgradesText;

pub fn upgrades_menu(mut commands: EntityCommands, asset_server: &AssetServer) {
    commands
        .insert(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/roboto.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(UpgradesText);
        });
}
