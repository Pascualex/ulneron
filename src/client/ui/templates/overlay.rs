use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::templates::{lobby_menu, upgrades_menu};

pub fn overlay(mut commands: EntityCommands, asset_server: &AssetServer) {
    commands
        .insert_bundle(NodeBundle {
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
                    lobby_menu(parent.spawn(), asset_server);
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
                    upgrades_menu(parent.spawn(), asset_server);
                });
        });
}
