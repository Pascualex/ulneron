use bevy::{prelude::*, window::close_on_esc};

use ulneron::{
    client::game::ClientGamePlugin,
    client::{
        controller::ClientControllerPlugin, graphics::ClientGraphicsPlugin, ui::ClientUiPlugin,
    },
    protocol::ProtocolPlugin,
    server::{
        controller::ServerControllerPlugin, game::ServerGamePlugin, lobby::ServerLobbyPlugin,
        networking::ServerNetworkingPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtocolPlugin)
        .add_plugin(ClientGamePlugin)
        .add_plugin(ClientGraphicsPlugin)
        .add_plugin(ClientUiPlugin)
        .add_plugin(ClientControllerPlugin)
        .add_plugin(ServerControllerPlugin)
        .add_plugin(ServerLobbyPlugin)
        .add_plugin(ServerGamePlugin)
        .add_plugin(ServerNetworkingPlugin)
        .add_system(close_on_esc)
        .run();
}
