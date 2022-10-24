use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::components::ResourcesText;

pub fn resources_menu(mut commands: EntityCommands, asset_server: &AssetServer) {
    commands.insert((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::TOP_RIGHT),
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        },
        ResourcesText,
    ));
}
