mod client;
mod client_endpoint;
mod events;
mod server;
mod server_endpoint;

use bevy::{prelude::*, window::close_on_esc};

use crate::{client::ClientPlugin, events::EventsPlugin, server::ServerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(ServerPlugin)
        .add_system(close_on_esc)
        .run();
}
