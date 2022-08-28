use bevy::{prelude::*, window::close_on_esc};

use ulneron::{
    client::game::ClientGamePlugin,
    client::{
        controller::ClientControllerPlugin, graphics::ClientGraphicsPlugin, ui::ClientLobbyPlugin,
    },
    networking::server::ServerNetworkingPlugin,
    protocol::ProtocolPlugin,
    server::ServerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtocolPlugin)
        .add_plugin(ClientGamePlugin)
        .add_plugin(ClientLobbyPlugin)
        .add_plugin(ClientControllerPlugin)
        .add_plugin(ClientGraphicsPlugin)
        .add_plugin(ServerPlugin)
        .add_plugin(ServerNetworkingPlugin)
        .add_system(close_on_esc)
        .run();
}
