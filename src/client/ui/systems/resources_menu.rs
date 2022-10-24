use bevy::prelude::*;

use crate::client::{
    game::{components::Resources, resources::GameState},
    graphics::components::LocalPlayer,
    ui::components::ResourcesText,
};

pub fn resources_menu(
    mut text_query: Query<&mut Text, With<ResourcesText>>,
    state: Res<GameState>,
    player_query: Query<&Resources, With<LocalPlayer>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = match *state {
        GameState::Waiting => "".to_string(),
        GameState::Running => match player_query.get_single() {
            Ok(resources) => {
                format!("{} nerite\n{} kills", resources.nerite, resources.kills,)
            }
            Err(_) => "No player".to_string(),
        },
    }
}
