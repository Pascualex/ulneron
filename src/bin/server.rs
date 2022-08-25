use bevy::{prelude::*, window::close_on_esc};

use zombie_bevy::{
    client::ClientPlugin, graphics::GraphicsPlugin, networking::server::ServerNetworkingPlugin,
    protocol::ProtocolPlugin, server::ServerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(ProtocolPlugin)
        .add_plugin(ServerPlugin)
        .add_plugin(ServerNetworkingPlugin)
        .add_system(close_on_esc)
        .run();
}
