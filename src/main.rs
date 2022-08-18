mod game;

use bevy::{prelude::*, window::close_on_esc};
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_system(close_on_esc)
        .run();
}
