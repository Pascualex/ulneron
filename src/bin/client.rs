use bevy::{prelude::*, window::close_on_esc};

use zombie_bevy::{
    client::ClientPlugin, client_endpoint::ClientEndpointPlugin, events::EventsPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ClientEndpointPlugin)
        .add_plugin(EventsPlugin)
        .add_system(close_on_esc)
        .run();
}
