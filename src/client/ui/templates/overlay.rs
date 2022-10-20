use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::templates::{lobby_menu, state_menu, upgrades_menu};

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
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
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
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    state_menu(parent.spawn(), asset_server);
                    upgrades_menu(parent.spawn(), asset_server);
                });
        });
}
