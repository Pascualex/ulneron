use bevy::{prelude::*, window::close_on_esc};

use zombie_bevy::{
    client::ClientPlugin, events::EventsPlugin, server::ServerPlugin,
    server_endpoint::ServerEndpointPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(ServerPlugin)
        .add_plugin(ServerEndpointPlugin)
        .add_system(close_on_esc)
        .run();
}
