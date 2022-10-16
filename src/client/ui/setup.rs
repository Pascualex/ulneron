use bevy::prelude::*;

use crate::client::ui::components::{LobbyText, UpgradesText};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(
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
                        .insert(LobbyText);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(
                            TextBundle::from_section(
                                "Upgrades",
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
                });
        });
}
