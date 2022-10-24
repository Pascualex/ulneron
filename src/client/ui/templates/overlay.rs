use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::client::ui::templates::{lobby_menu, resources_menu, upgrades_menu};

pub fn overlay(mut commands: EntityCommands, asset_server: &AssetServer) {
    let root = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    };
    let left_panel = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        ..default()
    };
    let right_panel = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        ..default()
    };
    commands.insert(root).with_children(|parent| {
        parent.spawn(left_panel).with_children(|parent| {
            lobby_menu(parent.spawn_empty(), asset_server);
        });
        parent.spawn(right_panel).with_children(|parent| {
            resources_menu(parent.spawn_empty(), asset_server);
            upgrades_menu(parent.spawn_empty(), asset_server);
        });
    });
}
