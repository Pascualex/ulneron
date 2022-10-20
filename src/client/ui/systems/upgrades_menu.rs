use bevy::prelude::*;

use crate::client::{
    game::{
        components::{Stats, Weapons},
        resources::GameState,
    },
    graphics::components::LocalPlayer,
    ui::components::UpgradesText,
};

pub fn upgrades_menu(
    mut text_query: Query<&mut Text, With<UpgradesText>>,
    state: Res<GameState>,
    player_query: Query<(&Stats, &Weapons), With<LocalPlayer>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = match *state {
        GameState::Waiting => "".to_string(),
        GameState::Running => match player_query.get_single() {
            Ok((stats, weapons)) => {
                format!(
                    "Stats\n  Speed: {:.2}\nWeapon\n  Fire rate: {:.2}\n  Range: {:.2}",
                    stats.speed,
                    1.0 / weapons.timer.duration().as_secs_f32(),
                    weapons.range,
                )
            }
            Err(_) => "No player".to_string(),
        },
    }
}
