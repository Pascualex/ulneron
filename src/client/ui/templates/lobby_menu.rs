use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::components::LobbyText;

pub fn lobby_menu(mut commands: EntityCommands, asset_server: &AssetServer) {
    commands
        .insert(
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
                size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                ..default()
            }),
        )
        .insert(LobbyText);
}
